pub mod router;

use std::rc::Rc;

use futures;
use futures::Future;

use hyper;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use self::router::{DirectoryRouter, Router};

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
            // TODO: have a proper 404.
            None => Box::new(futures::future::ok(
                Response::new()
                    .with_header(ContentLength(3))
                    .with_body("abc"),
            )),
        }
    }
}

pub trait RequestHandler {
    fn handle(&self, _req: Request) -> <WebService as Service>::Future;
}

pub fn default_router() -> Rc<Router> {
    Rc::new(DirectoryRouter::new())
}
