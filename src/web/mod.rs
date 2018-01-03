use futures;
use futures::Future;

use hyper;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

pub struct WebService;

impl WebService {
    pub fn new() -> WebService {
        WebService
    }
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(3))
                .with_body("abc")
        ))
    }
}
