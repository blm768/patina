pub mod handler;
pub mod router;
pub mod service;

use std::rc::Rc;

use self::handler::{DummyHandler, RequestHandler};
use self::router::{DirectoryRouter, Router};

pub fn default_router() -> Rc<Router> {
    Rc::new(DirectoryRouter::new().with_index(DummyHandler::new("testing".to_owned())))
}
