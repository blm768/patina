use futures;

use hyper::{Method, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::Service;

use mime;

use web::service::WebService;

pub type Request = <WebService as Service>::Request;
pub type Response = <WebService as Service>::Response;
pub type ResponseFuture = <WebService as Service>::Future;

// TODO: create a RequestContext trait?
// (would let us have a global error handler that renders an error page)

pub trait RequestHandler {
    fn handle(&self, request: Request) -> ResponseFuture;
}

// Lets plain functions and closures work as request handlers
impl<T> RequestHandler for T
where
    T: Fn(Request) -> ResponseFuture,
{
    fn handle(&self, request: Request) -> ResponseFuture {
        self(request)
    }
}

// This helps us build route trees without needing to explicitly box all the handlers.
impl<T: RequestHandler + 'static> From<T> for Box<RequestHandler> {
    fn from(handler: T) -> Box<RequestHandler> {
        Box::new(handler)
    }
}

/**
 * Handles a request by returning a static string
 */
pub struct DummyHandler {
    content: String,
}

impl DummyHandler {
    pub fn new(content: String) -> DummyHandler {
        DummyHandler { content: content }
    }
}

impl RequestHandler for DummyHandler {
    fn handle(&self, request: Request) -> ResponseFuture {
        match *request.method() {
            Method::Get => {
                let response = Response::new()
                    .with_header(ContentLength(self.content.len() as u64))
                    .with_body(self.content.clone());
                Box::new(futures::future::ok(response))
            }
            _ => {
                // TODO: do this properly.
                let mut response = Response::new()
                    .with_header(ContentLength("invalid method".len() as u64))
                    .with_header(ContentType(mime::TEXT_PLAIN))
                    .with_body("invalid method");
                response.set_status(StatusCode::BadRequest);
                Box::new(futures::future::ok(response))
            }
        }
    }
}
