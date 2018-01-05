/**
 * Contains basic HTTP request handlers
 */
pub mod handler;

/**
 * Helps map paths to request handlers
 */
pub mod router;

/**
 * Defines the main Web service for use with Hyper
 */
pub mod service;

use std::rc::Rc;

use self::handler::{DummyHandler, RequestHandler};
use self::router::{DirectoryRouter, Router};

/**
 * Builds and returns the application's default router
 */
pub fn default_router() -> Rc<Router> {
    Rc::new(
        DirectoryRouter::new()
            .with_index(DummyHandler::new("index page".to_owned()))
            .with_named_route(
                "test",
                DirectoryRouter::new()
                    .with_index(DummyHandler::new("testing".to_owned()))
                    .with_named_route("child", DummyHandler::new("child".to_owned())),
            ),
    )
}
