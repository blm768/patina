use hyper::{Method, StatusCode};

use web::handler;
use web::handler::{DummyHandler, Request, RequestHandler, ResponseFuture};
use web::service;

pub fn handle_transactions(request: Request) -> ResponseFuture {
    // TODO: handle properly.
    match *request.method() {
        Method::Get => DummyHandler::new("index of transactions".to_owned()).handle(request),
        _ => service::response_as_future(handler::error_response(StatusCode::MethodNotAllowed)),
    }
}
