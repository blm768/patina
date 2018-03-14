extern crate clap;
extern crate hyper;
extern crate patina;

use std::net::SocketAddr;
use std::rc::Rc;

use clap::{App, Arg};

use hyper::server::Http;

use patina::web;
use patina::web::service::WebService;

const DEFAULT_BIND_IP: &str = "0.0.0.0";
const DEFAULT_HTTP_PORT: u16 = 80;

pub fn main() {
    /*
     * Handle arguments
     */

    let app = App::new("patina")
        // Uses a trick from https://stackoverflow.com/questions/27840394/
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>"))
        .about("a Web-based personal finance tool")
        .author("Benjamin L. Merritt")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("the HTTP port to which the server will bind")
            .takes_value(true))
        .arg(Arg::with_name("bind")
            .short("b")
            .long("bind")
            .value_name("ADDRESS")
            .help("the IP address to which the server will bind")
            .takes_value(true));

    let matches = app.get_matches();

    let port = match matches.value_of("port") {
        Some(port_str) => port_str.parse().expect("invalid port number"),
        None => DEFAULT_HTTP_PORT,
    };
    let ip_addr = {
        let addr_string = matches.value_of("bind").unwrap_or(DEFAULT_BIND_IP);
        addr_string.parse().expect("invalid bind IP address")
    };
    let addr = SocketAddr::new(ip_addr, port);

    /*
     * Start server
     */

    {
        let router = web::default_router();

        let server = Http::new()
            .bind(&addr, move || Ok(WebService::new(Rc::clone(&router))))
            .unwrap();
        server.run().unwrap();
    }
}
