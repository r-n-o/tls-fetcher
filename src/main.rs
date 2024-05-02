use axum::{
    body::Body,
    http::{Method, Request}
};
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;

use std::path::PathBuf;
use tokio::net::UnixStream;

#[tokio::main]
async fn main() {
    // real goal here: fetch this over TLS
    // but for now: can we do HTTP only?
    //let url = "https://www.googleapis.com/oauth2/v3/certs"; // The URL path used in the actual request

    // TODO: this should be flexible and allow for VSOCK or UNIX sockets
    let socket_path = PathBuf::from("/tmp/host.sock");
    let stream = TokioIo::new(UnixStream::connect(socket_path).await.unwrap());

    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let request = Request::builder()
        .method(Method::GET)
        .uri("/socat/")
        .header("Host", "www.dest-unreach.org")
        .body(Body::empty())
        .unwrap();

    let response = sender.send_request(request).await.unwrap();
    println!("response {}", response.status());

    let body = response.collect().await.unwrap().to_bytes();
    let body = String::from_utf8(body.to_vec()).unwrap();
    println!("response body {}", body);
}
