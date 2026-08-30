use std::net::SocketAddr;
use std::sync::Arc;

use moq_rs::protocol::framing::{read_protocol_message, write_protocol_message};
use moq_rs::relay::Relay;
use moq_rs::tls::make_server_config;
use quinn::Endpoint;
use tokio::sync::{Mutex, mpsc};

const SERVER_ADDR: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let server_addr: SocketAddr = SERVER_ADDR.parse().expect("invalid server address");
    let server_config = make_server_config();
    let endpoint = Endpoint::server(server_config, server_addr).expect("failed to create endpoint");
    println!("Server listening on {server_addr}");

    let relay = Arc::new(Mutex::new(Relay::new()));
    while let Some(incoming) = endpoint.accept().await {
        let connection = match incoming.await {
            Ok(connection) => connection,
            Err(error) => {
                println!("failed to establish connection: {error}");
                continue;
            }
        };
        let relay = Arc::clone(&relay);
        tokio::spawn(async move {
            handle_client(connection, relay).await;
        });
    }
}

async fn handle_client(connection: quinn::Connection, relay: Arc<Mutex<Relay>>) {
    let (send, mut recv) = match connection.accept_bi().await {
        Ok(streams) => streams,
        Err(error) => {
            println!("failed to accept bidirectional stream: {error}");
            return;
        }
    };

    let (outbound_tx, outbound_rx) = mpsc::channel(32);
    let client_id = relay.lock().await.connect(outbound_tx);

    println!(
        "Client {client_id} connected from {}",
        connection.remote_address()
    );

    let mut writer_task = tokio::spawn(write_client_messages(send, outbound_rx));

    let result = tokio::select! {
        result = read_client_messages(&mut recv, client_id, &relay) => result,
        result = &mut writer_task => match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(error)) => Err(error),
            Err(error) => Err(error.into()),
        },
    };

    writer_task.abort();
    let _ = writer_task.await;
    relay.lock().await.disconnect(client_id);

    if let Err(error) = result {
        println!("Client {client_id} disconnected with an error: {error}");
    }
    println!("Client {client_id} disconnected");
}

async fn read_client_messages(
    recv: &mut quinn::RecvStream,
    client_id: moq_rs::relay::ClientId,
    relay: &Arc<Mutex<Relay>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(message) = read_protocol_message(recv).await? {
        let recipients = match relay.lock().await.handle_message(client_id, &message) {
            Ok(recipients) => recipients,
            Err(error) => {
                println!(
                    "Client {client_id} rejected {:?}: {error}",
                    message.message_type
                );
                continue;
            }
        };

        for recipient in recipients {
            if recipient.send(message.clone()).await.is_err() {
                println!("Client {client_id} could not forward message: recipient is disconnected");
            }
        }
    }

    Ok(())
}

async fn write_client_messages(
    mut send: quinn::SendStream,
    mut outbound_rx: mpsc::Receiver<moq_rs::protocol::message::Message>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(message) = outbound_rx.recv().await {
        write_protocol_message(&mut send, &message).await?;
    }

    send.finish()?;
    Ok(())
}
