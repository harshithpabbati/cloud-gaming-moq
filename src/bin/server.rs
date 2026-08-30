use std::net::SocketAddr;
use std::sync::Arc;

use moq_rs::protocol::framing::{read_protocol_message, write_protocol_message};
use moq_rs::protocol::handler::handle_message;
use moq_rs::relay::channel::ChannelManager;
use moq_rs::relay::client::ClientManager;
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

    let channel_manager = Arc::new(Mutex::new(ChannelManager::new()));
    let client_manager = Arc::new(Mutex::new(ClientManager::new()));
    while let Some(incoming) = endpoint.accept().await {
        let connection = incoming.await.expect("failed to establish connection");
        let channel_manager = Arc::clone(&channel_manager);
        let client_manager = Arc::clone(&client_manager);
        tokio::spawn(async move {
            handle_client(connection, channel_manager, client_manager).await;
        });
    }
}

async fn handle_client(
    connection: quinn::Connection,
    channel_manager: Arc<Mutex<ChannelManager>>,
    client_manager: Arc<Mutex<ClientManager>>,
) {
    let (send, mut recv) = connection
        .accept_bi()
        .await
        .expect("failed to open bidirectional stream");

    let (outbound_tx, outbound_rx) = mpsc::channel(32);
    let client_id = client_manager.lock().await.register(outbound_tx);

    println!(
        "Client {client_id} connected from {}",
        connection.remote_address()
    );

    let writer_task = tokio::spawn(async move {
        write_client_messages(send, outbound_rx).await;
    });

    while let Some(message) = read_protocol_message(&mut recv)
        .await
        .expect("failed to read protocol message")
    {
        handle_message(
            &message,
            client_id,
            channel_manager.as_ref(),
            client_manager.as_ref(),
        )
        .await;
    }

    client_manager.lock().await.remove(client_id);
    writer_task.await.expect("client writer task failed");

    println!("Client {client_id} disconnected");
}

async fn write_client_messages(
    mut send: quinn::SendStream,
    mut outbound_rx: mpsc::Receiver<moq_rs::protocol::message::Message>,
) {
    while let Some(message) = outbound_rx.recv().await {
        if let Err(error) = write_protocol_message(&mut send, &message).await {
            println!("failed to send protocol message to client: {error}");
            return;
        }
    }

    send.finish().expect("failed to finish stream");
}
