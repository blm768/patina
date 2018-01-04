use std::collections::HashMap;

use web::RequestHandler;

pub trait Router {
    fn route(&self, path: &str) -> Option<&RequestHandler>;
}

pub enum RouteEntry {
    Router(Box<Router>),
    Handler(Box<RequestHandler>),
}

/**
 * Performs routing at the directory level
 */
#[derive(Default)]
pub struct DirectoryRouter {
    // TODO: support an index entry.
    static_members: HashMap<Box<str>, RouteEntry>,
    // TODO: support regex entries?
    // TODO: support a default router?
}

impl DirectoryRouter {
    pub fn new() -> DirectoryRouter {
        DirectoryRouter {
            static_members: HashMap::new(),
        }
    }

    pub fn static_route(mut self, name: &str, entry: RouteEntry) -> DirectoryRouter {
        self.static_members.insert(Box::from(name), entry);
        self
    }
}

impl Router for DirectoryRouter {
    fn route(&self, path: &str) -> Option<&RequestHandler> {
        // Trim leading slash.

        // TODO: make sure there is a child!
        // Look up the child.
        let (head, tail) = {
            match path.find('/') {
                Some(separator) => path.split_at(separator),
                None => (path, ""),
            }
        };
        // TODO: implement.
        None
    }
}
