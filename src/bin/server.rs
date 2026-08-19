use std::net::SocketAddr;

use moq_rs::protocol::framing::{read_message, write_message};
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

    while let Some(incoming) = endpoint.accept().await {
        println!("Incoming connection...");

        let connection = incoming.await.expect("failed to establish connection");

        println!("Connected to {}", connection.remote_address());

        let (mut send, mut recv) = connection
            .accept_bi()
            .await
            .expect("failed to open bidirectional stream");

        while let Some(message) = read_message(&mut recv)
            .await
            .expect("failed to read message")
        {
            println!("Received: {}", String::from_utf8_lossy(&message));

            write_message(&mut send, &message)
                .await
                .expect("failed to echo message");
        }

        send.finish().expect("failed to finish stream");
        println!("Echo complete, keeping connection alive");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
