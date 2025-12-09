//! A library for tessellation fill algorithm.
//!
//! This fills a polygonal area like a flood fill, but using a tessellation approach.
//! This is useful if vast areas of a polygon is empty, and traditional flood fill
//! would be too slow or memory intensive.

mod errors;
pub mod models;
pub use errors::TessellationFillError;
pub mod traits;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
