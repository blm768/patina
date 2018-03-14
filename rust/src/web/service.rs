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

/**
 * A `hyper` service implementation for the application's main Web server
 */
impl WebService {
    pub fn new(router: Rc<Router>) -> WebService {
        WebService { router: router }
    }
}

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    // TODO: use a different error type? (maybe use the failure crate...)
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

pub fn response_as_future(
    response: <WebService as Service>::Response,
) -> <WebService as Service>::Future {
    Box::new(futures::future::ok(response))
}
