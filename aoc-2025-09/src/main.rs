//! This code as it stands will produce the correct answer for the given input,
//! but is fundamentally flawed and does not correctly solve the problem as stated.
//!
//! The issue lies in the way visibility bounds are calculated for each point.
//! See [`visibility`] module for more details.

pub mod colour;
mod compare;
pub mod models;
use compare::*;
mod parse;
use parse::*;
mod input;
pub use input::INPUT;
pub mod visibility;

#[cfg(feature = "profile")]
use std::time::Instant;

fn main() {
    let indexed_coords = visibility::build_visibility_bounds_for_indexed_coords(
        indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords"),
    );

    #[cfg(feature = "profile")]
    let start = Instant::now();
    let best_rectangle_within_polygon =
        find_best_match(&indexed_coords, |a, b| compare_area_with_visibility(a, b))
            .expect("Error finding best match with visibility")
            .expect("No rectangle found within polygon");

    #[cfg(feature = "profile")]
    {
        let duration = start.elapsed();
        eprintln!("Time elapsed in finding best match: {:?}", duration);
    }

    println!(
        "Best rectangle within polygon area: {}",
        best_rectangle_within_polygon.area()
    );
}

#[cfg(test)]
mod test_part_1 {
    use super::*;

    const INPUT: &str = "7,1
                         11,1
                         11,7
                         9,7
                         9,5
                         2,5
                         2,3
                         7,3";

    #[test]
    fn test_find_largest_area() {
        let indexed_coords =
            indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");
        let best_rectangle = find_best_match(&indexed_coords, |a, b| Ok(a.area().cmp(&b.area())))
            .expect("Error finding best match")
            .expect("No rectangle found");
        assert_eq!(best_rectangle.area(), 50);
    }
}
