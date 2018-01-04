use std::borrow::Borrow;
use std::collections::HashMap;

use web::RequestHandler;

/**
 * Maps a request to an object that can handle it
 */
pub trait Router {
    /**
     * Returns a reference to the RequestHandler that should handle the given request (if one exists)
     */
    fn route(&self, path: &str) -> Option<&RequestHandler>;
}

/**
 * An entry in a table of routing handlers
 */
pub enum RouteEntry {
    /// A sub-router (typically used to handle nested directories)
    Router(Box<Router>),
    /// A request handler
    Handler(Box<RequestHandler>),
}

impl<T: RequestHandler + 'static> From<T> for RouteEntry {
    fn from(handler: T) -> RouteEntry {
        RouteEntry::Handler(Box::new(handler))
    }
}

/**
 * Performs routing at the directory level
 */
#[derive(Default)]
pub struct DirectoryRouter {
    index_handler: Option<Box<RequestHandler>>,
    named_routes: HashMap<Box<str>, RouteEntry>,
    // TODO: support regex entries?
    // TODO: support a default router?
}

impl DirectoryRouter {
    pub fn new() -> DirectoryRouter {
        DirectoryRouter {
            index_handler: None,
            named_routes: HashMap::new(),
        }
    }

    pub fn with_index<T: Into<Box<RequestHandler>>>(mut self, handler: T) -> DirectoryRouter {
        self.index_handler = Some(handler.into());
        self
    }

    pub fn with_named_route<T: Into<RouteEntry>>(
        mut self,
        name: &str,
        entry: T,
    ) -> DirectoryRouter {
        self.named_routes.insert(Box::from(name), entry.into());
        self
    }
}

impl Router for DirectoryRouter {
    fn route(&self, path: &str) -> Option<&RequestHandler> {
        // Trim leading slash.

        let (head, tail) = match path.find('/') {
            Some(separator) => path.split_at(separator),
            None => (path, ""),
        };

        if head.len() == 0 {
            // TODO: make sure there's no tail?
            match self.index_handler {
                Some(ref handler) => Some(handler.borrow()),
                None => None,
            }
        } else {
            // TODO: implement.
            None
        }
    }
}
