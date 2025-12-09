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

const CACHE_FILE: &str = "grid_fill_outside.txt";

#[cfg(feature = "profile")]
use std::time::Instant;

fn main() {
    let indexed_coords = visibility::build_visibility_bounds_for_indexed_coords(
        indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords"),
    );
    // let coords: Vec<models::Coords> = indexed_coords.iter().map(|ic| ic.coords).collect();

    // #[cfg(feature = "profile")]
    // let start = Instant::now();

    // let grid = if let Ok(grid) = colour::Grid::load_from(std::path::Path::new(CACHE_FILE)) {
    //     grid
    // } else {
    //     let mut grid = colour::Grid::new_to_fit(coords.iter(), colour::Colour::Colourless);
    //     grid.boundary(&coords);
    //     grid.fill_from(0, 0, colour::Colour::White);
    //     grid.save_to(std::path::Path::new(CACHE_FILE))
    //         .expect("Failed to save grid to cache file");
    //     grid
    // };

    // assert_eq!(grid.get(0, 0), Some(colour::Colour::Colourless));
    // #[cfg(feature = "profile")]
    // {
    //     let duration = start.elapsed();
    //     println!("Time taken to colouring grid() is: {:?}", duration);
    // }
    // println!(
    //     "Colourless count: {}",
    //     grid.colour_count(colour::Colour::Colourless)
    // );
    // println!("White count: {}", grid.colour_count(colour::Colour::White));

    // let best_rectangle =
    //     find_best_match(&indexed_coords, move |    candidate, current| match candidate.area().cmp(&current.area()) {
    //         std::cmp::Ordering::Greater => {
    //             if grid
    //                 .check_rectangle_border(candidate, |colour| {
    //                     colour.is_some() && colour != Some(colour::Colour::White)
    //                 })
    //             {
    //                 #[cfg(feature = "trace")]
    //                 {
    //                     eprintln!(
    //                         "Candidate rectangle with area {} is bigger than current with area {} and is within the polygon",
    //                         candidate.area(),
    //                         current.area()
    //                     );
    //                 }
    //                 Ok(std::cmp::Ordering::Greater)
    //             } else {
    //                 #[cfg(feature = "trace")]
    //                 {
    //                     eprintln!(
    //                         "Candidate rectangle with area {} is bigger than current with area {} but is NOT within the polygon",
    //                         candidate.area(),
    //                         current.area()
    //                     );
    //                 }
    //                 Ok(std::cmp::Ordering::Less)
    //             }
    //         }
    //         ord => Ok(ord),
    //     })
    //     .expect("Error finding best match")
    //     .expect("No rectangle found");

    // dbg!(best_rectangle);
    // println!("Best rectangle area: {}", best_rectangle.area());

    let best_rectangle_within_polygon =
        find_best_match(&indexed_coords, |a, b| compare_area_with_visibility(a, b))
            .expect("Error finding best match with visibility")
            .expect("No rectangle found within polygon");

    println!(
        "Best rectangle within polygon area: {}",
        best_rectangle_within_polygon.area()
    );
    assert_eq!(best_rectangle_within_polygon.bounding.1, 94710);
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
