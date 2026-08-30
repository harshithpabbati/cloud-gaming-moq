use std::error::Error;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use moq_rs::protocol::framing::{read_protocol_message, write_protocol_message};
use moq_rs::protocol::message::{Message, MessageType};
use quinn::{ClientConfig, Connection, Endpoint, RecvStream, SendStream};
use rustls::pki_types::CertificateDer;

const SERVER_ADDR: &str = "127.0.0.1:5000";
const CERT_PATH: &str = "certs/server.der";
const CHANNEL_NAME: &str = "game-123";

struct RelayClient {
    _endpoint: Endpoint,
    _connection: Connection,
    send: SendStream,
    recv: RecvStream,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let server_addr = SERVER_ADDR.parse::<SocketAddr>()?;
    let cert_bytes = fs::read(CERT_PATH)?;

    let mut subscriber = connect(server_addr, &cert_bytes).await?;
    write_protocol_message(
        &mut subscriber.send,
        &Message {
            message_type: MessageType::Subscribe,
            channel_name: CHANNEL_NAME.to_string(),
            payload: Vec::new(),
        },
    )
    .await?;

    // The protocol has no subscription acknowledgement yet, so allow the relay to register it.
    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut publisher = connect(server_addr, &cert_bytes).await?;
    write_protocol_message(
        &mut publisher.send,
        &Message {
            message_type: MessageType::Publish,
            channel_name: CHANNEL_NAME.to_string(),
            payload: Vec::new(),
        },
    )
    .await?;

    for frame_number in 1..=3 {
        write_protocol_message(
            &mut publisher.send,
            &Message {
                message_type: MessageType::Data,
                channel_name: CHANNEL_NAME.to_string(),
                payload: format!("frame {frame_number}").into_bytes(),
            },
        )
        .await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    for _ in 1..=3 {
        let message = read_protocol_message(&mut subscriber.recv)
            .await?
            .ok_or("relay closed the subscriber stream")?;
        println!(
            "Received {:?} on '{}': {}",
            message.message_type,
            message.channel_name,
            String::from_utf8_lossy(&message.payload)
        );
    }

    publisher.send.finish()?;
    subscriber.send.finish()?;
    Ok(())
}

async fn connect(
    server_addr: SocketAddr,
    cert_bytes: &[u8],
) -> Result<RelayClient, Box<dyn Error + Send + Sync>> {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add(CertificateDer::from(cert_bytes.to_vec()))?;

    let client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let client_config = ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)?,
    ));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let connection = endpoint.connect(server_addr, "localhost")?.await?;
    let (send, recv) = connection.open_bi().await?;

    Ok(RelayClient {
        _endpoint: endpoint,
        _connection: connection,
        send,
        recv,
    })
}
