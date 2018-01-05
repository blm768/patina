extern crate futures;
extern crate hyper;

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
