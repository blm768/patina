use std::borrow::Borrow;
use std::collections::HashMap;

use web::RequestHandler;

/// Removes a single leading slash (if present) from a string
pub fn remove_leading_slash(path: &str) -> &str {
    if path.starts_with('/') {
        &path[1..]
    } else {
        path
    }
}

/**
 * Maps a request to an object that can handle it
 */
pub trait Router {
    /**
     * Returns a reference to the RequestHandler that should handle the given request (if one exists)
     */
    fn route(&self, path: &str) -> Option<&RequestHandler>;
}

// We let RequestHandlers route to themselves if given an empty path.
// This makes building routing trees simpler.
impl<T: RequestHandler> Router for T {
    fn route(&self, path: &str) -> Option<&RequestHandler> {
        if remove_leading_slash(path).is_empty() {
            Some(self)
        } else {
            None
        }
    }
}

// This helps us build route trees without needing to explicitly box all the routers.
impl<T: Router + 'static> From<T> for Box<Router> {
    fn from(router: T) -> Box<Router> {
        Box::new(router)
    }
}

/**
 * Performs routing at the directory level
 *
 * This router finds the first element of the given path and uses it to dispatch
 * to the correct "sub-router", which is passed the remaining elements of the path.
 */
#[derive(Default)]
pub struct DirectoryRouter {
    index_handler: Option<Box<RequestHandler>>,
    named_routes: HashMap<Box<str>, Box<Router>>,
    // TODO: support regex entries?
    // TODO: support a default router?
}

impl DirectoryRouter {
    /// Constructs a DirectoryRouter with no routing entries
    pub fn new() -> DirectoryRouter {
        DirectoryRouter {
            index_handler: None,
            named_routes: HashMap::new(),
        }
    }

    /**
     * Sets the "index" handler, which handles requests for the directory itself (like the traditional "index.html")
     */
    pub fn with_index<T: Into<Box<RequestHandler>>>(mut self, handler: T) -> DirectoryRouter {
        self.index_handler = Some(handler.into());
        self
    }

    /**
     * Adds a named child route
     *
     * ```
     * let router = DirectoryRouter::new().with_named_route("myroute", DummyHandler::new("myroute"));
     * let handler = router.route("/myroute/");
     * ```
     */
    pub fn with_named_route<T: Into<Box<Router>>>(
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
        let path_trimmed = remove_leading_slash(path);

        let (head, tail) = match path_trimmed.find('/') {
            Some(separator) => path_trimmed.split_at(separator),
            None => (path_trimmed, ""),
        };

        // Are we retrieving the index page?
        if head.is_empty() {
            if tail.is_empty() {
                match self.index_handler {
                    Some(ref handler) => Some(handler.borrow()),
                    None => None,
                }
            } else {
                None
            }
        } else {
            match self.named_routes.get(head) {
                Some(router) => router.route(tail),
                None => None,
            }
        }
    }
}
