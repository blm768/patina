/**
 * Contains basic HTTP request handlers
 */
pub mod handler;

/**
 * Integrates request handlers with resources
 */
pub mod resource;

/**
 * Helps map paths to request handlers
 */
pub mod router;

/**
 * Defines the main Web service for use with Hyper
 */
pub mod service;

use std::rc::Rc;

//use hyper::Method;

use self::handler::{DummyHandler, Request, RequestHandler, ResponseFuture};
use self::router::{DirectoryRouter, Router};

/*
NOTE:
Request handling procedure:
    TODO: handle PATCH?
No child path:
    GET: index ("singleton get"?)
    POST: create ("singleton create-child?")
    PUT: singleton update
    DELETE: singleton delete
Child path:
    GET: get
    POST: N/A
    PUT: update
    DELETE: delete
Child path with more stuff:
*/

fn handle_transactions(request: Request) -> ResponseFuture {
    // TODO: handle properly.
    DummyHandler::new("transactions".to_owned()).handle(request)
    /*
    match request.method() {
        Method::Get => {},
        _ => {}
    }
    */
}

/**
 * Builds and returns the application's default router
 */
pub fn default_router() -> Rc<Router> {
    Rc::new(
        DirectoryRouter::new()
            .with_index(DummyHandler::new("index page".to_owned()))
            .with_named_route(
                "transactions",
                DirectoryRouter::new().with_index(handle_transactions),
            ),
    )
}
