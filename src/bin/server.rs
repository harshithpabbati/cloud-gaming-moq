use std::net::SocketAddr;

use moq_rs::protocol::framing::read_protocol_message;
use moq_rs::protocol::handler::handle_message;
use moq_rs::relay::channel::ChannelManager;
use moq_rs::tls::make_server_config;
use quinn::Endpoint;

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

    let mut channel_manager = ChannelManager::new();
    let mut next_client_id = 0;

    while let Some(incoming) = endpoint.accept().await {
        println!("Incoming connection...");

        let connection = incoming.await.expect("failed to establish connection");
        let client_id = next_client_id;
        next_client_id += 1;

        println!("Connected to {}", connection.remote_address());

        let (mut send, mut recv) = connection
            .accept_bi()
            .await
            .expect("failed to open bidirectional stream");

        while let Some(message) = read_protocol_message(&mut recv)
            .await
            .expect("failed to read protocol message")
        {
            handle_message(&message, client_id, &mut channel_manager).await;
        }

        send.finish().expect("failed to finish stream");
        println!("Echo complete, keeping connection alive");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
