use std::fs;
use std::path::Path;

use quinn::ServerConfig;
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

const CERT_PATH: &str = "certs/server.der";
const KEY_PATH: &str = "certs/server.key";

pub fn load_or_generate_certificate() -> (CertificateDer<'static>, PrivateKeyDer<'static>) {
    if Path::new(CERT_PATH).exists() && Path::new(KEY_PATH).exists() {
        let cert = fs::read(CERT_PATH).expect("failed to read certificate");

        let key = fs::read(KEY_PATH).expect("failed to read private key");

        return (CertificateDer::from(cert), PrivateKeyDer::Pkcs8(key.into()));
    }

    println!("Generating self-signed certificate...");

    let cert = generate_simple_self_signed(vec!["localhost".to_string()])
        .expect("failed to generate certificate");

    let cert_der = cert.cert.der().clone();
    let key_der = cert.signing_key.serialize_der();

    fs::create_dir_all("certs").expect("failed to create cert directory");

    fs::write(CERT_PATH, cert_der.as_ref()).expect("failed to write certificate");

    fs::write(KEY_PATH, &key_der).expect("failed to write private key");

    println!("Generated certificate in {CERT_PATH}");

    (cert_der, PrivateKeyDer::Pkcs8(key_der.into()))
}

pub fn make_server_config() -> ServerConfig {
    let (cert, key) = load_or_generate_certificate();

    ServerConfig::with_single_cert(vec![cert], key).expect("failed to create server config")
}
