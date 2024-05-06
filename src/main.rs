use std::path::PathBuf;

use std::io::ErrorKind;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::str;

use rustls::RootCertStore;

/// Function to fetch TLS content
/// - `host` is the FQDN ("www.googleapis.com")
/// - `path` is the absolute resource path _with_ leading slash ("/oauth2/v3/certs")
fn fetch_tls_content(socket_path: PathBuf, config: rustls::ClientConfig, host: &str, path: &str) -> Vec<u8> {
    let server_name = "www.googleapis.com".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();

    // This would be the normal way to connect:
    //     let mut sock = TcpStream::connect("<host>:<port>").unwrap();
    // We're instead going through our local (unix) socket which proxies to the right host on port 443 for us (socket proxy)
    // TODO: this should be flexible and allow for VSOCK or UNIX sockets; we'll want unix socket locally and vsock in production.
    let mut sock = UnixStream::connect(socket_path).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    
    let http_request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    println!("=== making HTTP request: \n{http_request}");    

    tls.write_all(http_request.as_bytes()).unwrap();
    let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    
    println!("=== current ciphersuite: {:?}", ciphersuite.suite());    
    let mut response_bytes = Vec::new();
    match tls.read_to_end(&mut response_bytes) {
        Ok(_) => response_bytes,
        Err(e) => {
            // Ignore eof errors: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof
            if e.kind() == ErrorKind::UnexpectedEof {
                return response_bytes
            }
            panic!("Unexpected error while reading TLS response: {}", e);
        }
    }
}

fn main() {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };
    
    let config: rustls::ClientConfig = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();

    let response_bytes = fetch_tls_content(PathBuf::from("/tmp/host.sock"), config, "www.googleapis.com", "/oauth2/v3/certs");
    println!("Got a response: \n{}", str::from_utf8(&response_bytes).unwrap());
}
