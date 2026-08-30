use std::net::SocketAddr;
use std::sync::Arc;

use moq_rs::protocol::framing::read_protocol_message;
use moq_rs::protocol::handler::handle_message;
use moq_rs::relay::channel::{ChannelManager, ClientId};
use moq_rs::relay::client::ClientManager;
use moq_rs::tls::make_server_config;
use quinn::Endpoint;
use tokio::sync::Mutex;

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

    let mut next_client_id = 0;

    while let Some(incoming) = endpoint.accept().await {
        println!("Incoming connection...");

        let connection = incoming.await.expect("failed to establish connection");
        let client_id = next_client_id;
        next_client_id += 1;

        let channel_manager = Arc::clone(&channel_manager);
        let client_manager = Arc::clone(&client_manager);

        tokio::spawn(async move {
            handle_client(connection, client_id, channel_manager, client_manager).await;
        });
    }
}

async fn handle_client(
    connection: quinn::Connection,
    client_id: ClientId,
    channel_manager: Arc<Mutex<ChannelManager>>,
    client_manager: Arc<Mutex<ClientManager>>,
) {
    client_manager.lock().await.register(client_id);

    println!(
        "Client {client_id} connected from {}",
        connection.remote_address()
    );

    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .expect("failed to open bidirectional stream");

    while let Some(message) = read_protocol_message(&mut recv)
        .await
        .expect("failed to read protocol message")
    {
        let mut channel_manager = channel_manager.lock().await;
        handle_message(&message, client_id, &mut channel_manager).await;
    }

    send.finish().expect("failed to finish stream");
    client_manager.lock().await.remove(client_id);

    println!("Client {client_id} disconnected");
}
