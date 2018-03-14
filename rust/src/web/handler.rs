use hyper::{Method, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::Service;

use mime;

use web::service;
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
                service::response_as_future(response)
            }
            _ => service::response_as_future(error_response(StatusCode::MethodNotAllowed)),
        }
    }
}

/**
 * A minimal request handler that returns a fixed status code
 */
pub struct BasicStatusCodeHandler {
    status: StatusCode,
}

impl BasicStatusCodeHandler {
    pub fn new(status: StatusCode) -> BasicStatusCodeHandler {
        BasicStatusCodeHandler { status: status }
    }
}

impl RequestHandler for BasicStatusCodeHandler {
    fn handle(&self, _request: Request) -> ResponseFuture {
        service::response_as_future(error_response(self.status))
    }
}

/**
 * Constructs a very basic error response for a given HTTP error code
 */
pub fn error_response(status: StatusCode) -> Response {
    let body = status.canonical_reason().unwrap_or("unknown error");
    let mut response = Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_header(ContentType(mime::TEXT_PLAIN))
        .with_body(body);
    response.set_status(status);

    response
}
