use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;

use moq_rs::protocol::framing::{read_message, write_message};
use quinn::{ClientConfig, Endpoint};
use rustls::pki_types::CertificateDer;

const SERVER_ADDR: &str = "127.0.0.1:5000";
const CERT_PATH: &str = "certs/server.der";

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let server_addr: SocketAddr = SERVER_ADDR.parse().expect("invalid server address");

    let cert_bytes = fs::read(CERT_PATH).expect("failed to read server certificate");

    let cert = CertificateDer::from(cert_bytes);

    let mut root_store = rustls::RootCertStore::empty();

    root_store
        .add(cert)
        .expect("failed to add server certificate");

    let client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let client_config = ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)
            .expect("failed to create QUIC client config"),
    ));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse().expect("invalid client address"))
        .expect("failed to create client endpoint");

    endpoint.set_default_client_config(client_config);

    println!("Connecting to {server_addr}...");

    let connection = endpoint
        .connect(server_addr, "localhost")
        .expect("failed to start connection")
        .await
        .expect("failed to establish connection");

    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .expect("failed to open bidirectional stream");

    let messages: &[&[u8]] = &[b"Hello", b"World", b"MoQ", b"", b"a"];

    for message in messages {
        write_message(&mut send, message)
            .await
            .expect("failed to send message");
    }

    send.finish().expect("failed to finish stream");

    while let Some(message) = read_message(&mut recv)
        .await
        .expect("failed to read response")
    {
        println!(
            "Received from server: {}",
            String::from_utf8_lossy(&message)
        );
    }
}
