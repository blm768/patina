pub mod handler;
pub mod router;
pub mod service;

use std::rc::Rc;

use self::handler::RequestHandler;
use self::router::{DirectoryRouter, Router};

pub fn default_router() -> Rc<Router> {
    Rc::new(DirectoryRouter::new())
}
