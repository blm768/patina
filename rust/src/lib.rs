#[macro_use]
extern crate diesel;
extern crate futures;
extern crate hyper;
extern crate mime;

/**
 * Handles database schema and interaction
 */
pub mod database;

/**
 * Defines useful primitives for data resources
 */
pub mod resource;

/**
 * Code to handle Web requests
 */
pub mod web;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
