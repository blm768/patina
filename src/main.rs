extern crate hyper;
extern crate patina;

use std::net::SocketAddr;

use hyper::server::Http;

use patina::web::WebService;

pub fn main() {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(WebService::new())).unwrap();
    server.run().unwrap();
}
