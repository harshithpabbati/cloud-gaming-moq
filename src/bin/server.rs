use std::net::SocketAddr;

use quinn::{Endpoint, ServerConfig};
use rcgen::generate_simple_self_signed;

const SERVER_ADDR: &str = "127.0.0.1:5000";

fn make_server_config() -> ServerConfig {
    let cert = generate_simple_self_signed(vec!["localhost".to_string()])
        .expect("failed to generate certificate");

    let key = rustls::pki_types::PrivateKeyDer::Pkcs8(
        cert.signing_key.serialize_der().into(),
    );

    ServerConfig::with_single_cert(
        vec![cert.cert.der().clone()],
        key,
    )
    .expect("failed to create server config")
}

#[tokio::main]
async fn main() {
    let server_addr: SocketAddr = SERVER_ADDR
        .parse()
        .expect("invalid server address");

    let server_config = make_server_config();

    let endpoint = Endpoint::server(server_config, server_addr)
        .expect("failed to create endpoint");

    println!("Server listening on {server_addr}");

    while let Some(incoming) = endpoint.accept().await {
        println!("Incoming connection...");

        let connection = incoming
            .await
            .expect("failed to establish connection");

        println!("Connected to {}", connection.remote_address());
    }
}
