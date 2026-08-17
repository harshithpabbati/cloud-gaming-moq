use std::net::SocketAddr;

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

        loop {
            match recv.read_chunk(1024, true).await {
                Ok(Some(chunk)) => {
                    let data = chunk.bytes;

                    println!("Received: {:?}", data);

                    send.write_all(&data).await.expect("failed to echo message");
                }

                Ok(None) => {
                    break;
                }

                Err(error) => {
                    panic!("failed to read from stream: {error}");
                }
            }
        }

        send.finish().expect("failed to finish stream");
        println!("Echo complete, keeping connection alive");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
