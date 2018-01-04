use futures;

use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::Service;

use web::service::WebService;

type Request = <WebService as Service>::Request;
type Response = <WebService as Service>::Response;

// TODO: create a RequestContext trait? (or RoutingContext?)

pub trait RequestHandler {
    fn handle(&self, request: Request) -> <WebService as Service>::Future;
}

pub struct DummyHandler {
    content: String,
}

impl DummyHandler {
    pub fn new(content: String) -> DummyHandler {
        DummyHandler { content: content }
    }
}

impl RequestHandler for DummyHandler {
    fn handle(&self, request: Request) -> <WebService as Service>::Future {
        match request.method() {
            &Method::Get => {
                let response = Response::new()
                    .with_header(ContentLength(self.content.len() as u64))
                    .with_body(self.content.clone());
                Box::new(futures::future::ok(response))
            }
            _ => {
                // TODO: do this properly.
                let mut response = Response::new()
                    .with_header(ContentLength("invalid method".len() as u64))
                    .with_body("invalid method");
                response.set_status(StatusCode::BadRequest);
                Box::new(futures::future::ok(response))
            }
        }
    }
}
