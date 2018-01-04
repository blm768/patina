use std::rc::Rc;

use futures;
use futures::Future;

use hyper;
use hyper::StatusCode;
use hyper::server::{Request, Response, Service};

use web::router::Router;

pub struct WebService {
    router: Rc<Router>,
}

impl WebService {
    pub fn new(router: Rc<Router>) -> WebService {
        WebService { router: router }
    }
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        match self.router.route(request.path()) {
            Some(handler) => handler.handle(request),
            None => {
                // TODO: include a helpful body with the 404? (unless it's an API request...)
                let mut response = Response::new();
                response.set_status(StatusCode::NotFound);
                Box::new(futures::future::ok(response))
            }
        }
    }
}
