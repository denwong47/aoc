//! ## Visibility Module
//!
//! This module provides functionality to determine visibility bounds
//! from a given point within a polygon defined by a set of indexed coordinates.
//! It includes functions to check if a line blocks visibility,
//! find visibility bounds, and build visibility bounds for a list of indexed coordinates.
//!
//! ## Warning
//!
//! This module does not work:
//!
//! ```text
//! ..............
//! .......#XXX#..
//! .......XXXXX..
//! ..AXXXXBXXXC..
//! ..XXXXXXXXXX..
//! ..#XXXXXX#XX..
//! .........XXX..
//! .........#X#..
//! ..............
//! ```
//!
//! A is at (2,3), B is at (7,3), C is at (11,3).
//!
//! Since visibility lines has no concept of "inside" or "outside" the polygon,
//! it consider B to be blocking A's visibility to the right, while this is only
//! true if the polygon is flipped:
//!
//! ```text
//! XXXXXXXXXXXXXX
//! XXXXXXX#XXX#XX
//! XXXXXXXX...XXX
//! XXAXXXXB...CXX
//! XXX........XXX
//! XX#XXXXXX#.XXX
//! XXXXXXXXXX.XXX
//! XXXXXXXXX#X#XX
//! XXXXXXXXXXXXXX
//! ```
//!
//! In other words, the visibility from A to C is dependant on whether the polygon is
//! considered to be the "X" area or the "." area; but this module has no concept of that.
//!
//! Some of this problem was mitigated by using two visibility bounds (absolute and corner):
//! if we hit a corner instead of a straight edge, we treat that as a weaker form of blocking;
//! and behave the opposite way when combining the two bounds:
//!
//! - for absolute bounds, we take the closest blocking edge we find; but
//! - for corner bounds, we take the furthest blocking edge we find.
//!
//! This however is not sufficient to solve the problem in all cases:
//!
//! ```text
//! ..............
//! .AXXXXXXXXXX#.
//! .BXXXXXX#XXXX.
//! ........XXXXX.
//! .CXXXXXX#XXXX.
//! .DXXXXXXXXXX#.
//! ..............
//! ```
//! In the above example, A only encounters corners when looking downwards. Using our logic
//! above, it sees B, C and D, and prefers D as the bottom visibility bound. However,
//! in reality, there is a gap after B that C and D should not be visible.
//!
//! Without knowing which side of the polygon is "inside" or "outside", this cannot be
//! correctly resolved.
//!
//! In actual fact, our input happens to not trigger this limitation, so the answer
//! is still correct, but this is **absolute garbage code** that only works by coincidence.

use crate::models::{Coords, IndexedCoords, VisibilityBounds};
use itertools::Itertools;


#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

/// Determines if a line defined by two [`IndexedCoords`] blocks visibility from a given point.
///
/// The line must be orthogonal (either horizontal or vertical).
fn blocks_visibility(from: &Coords, line: [&IndexedCoords; 2]) -> Option<(Direction, bool)> {
    let (line_a, line_b) = (line[0].coords, line[1].coords);

    let is_vertical_line = line_a[0] == line_b[0];
    let is_horizontal_line = line_a[1] == line_b[1];

    let horizontal_bounding = (line_a[0].min(line_b[0]), line_a[0].max(line_b[0]));
    let vertical_bounding = (line_a[1].min(line_b[1]), line_a[1].max(line_b[1]));

    // Check if `from` is the horizontal bounding box defined
    match from {
        // Horizontally bounded
        _ if (is_horizontal_line
            && from[0] >= horizontal_bounding.0
            && from[0] <= horizontal_bounding.1) =>
        {
            let is_corner = from[0] == horizontal_bounding.0 || from[0] == horizontal_bounding.1;
            // For some reason, the y-axis is inverted in this coordinate system
            if from[1] > vertical_bounding.0 {
                Some((Direction::Up, is_corner))
            } else if from[1] < vertical_bounding.0 {
                Some((Direction::Down, is_corner))
            } else {
                // On the line
                None
            }
        }
        _ if (is_vertical_line
            && from[1] >= vertical_bounding.0
            && from[1] <= vertical_bounding.1) =>
        {
            let is_corner = from[1] == vertical_bounding.0 || from[1] == vertical_bounding.1;
            if from[0] > horizontal_bounding.0 {
                Some((Direction::Left, is_corner))
            } else if from[0] < horizontal_bounding.0 {
                Some((Direction::Right, is_corner))
            } else {
                // On the line
                None
            }
        }
        _ => None,
    }
}

/// Finds the visibility bounds from a given coordinate within
/// a polygon defined by the [`IndexedCoords`].
///
/// This does not mean the point is inside the polygon - this simply evaluates
/// the orthogonal visibility bounds based on the polygon edges.
pub fn find_visibility_bounds(from: &Coords, polygon: &[IndexedCoords]) -> VisibilityBounds {
    let (absolute_bounds, corner_bounds) = polygon.iter().circular_tuple_windows().fold(
        (VisibilityBounds::default(), VisibilityBounds::default()),
        |(mut absolute_bounds, mut corner_bounds), (line_a, line_b)| {
            if let Some((direction, is_corner)) = blocks_visibility(from, [line_a, line_b]) {
                let bounds = if is_corner {
                    &mut corner_bounds
                } else {
                    &mut absolute_bounds
                };

                match direction {
                    Direction::Left => {
                        // The smaller x coordinate is the left bound,
                        // we are looking at visibility afterall.
                        bounds.left = bounds.left.map_or_else(
                            || Some(line_a.coords[0]),
                            |v| {
                                Some(if is_corner {
                                    v.min(line_a.coords[0])
                                } else {
                                    v.max(line_a.coords[0])
                                })
                            },
                        );
                    }
                    Direction::Right => {
                        bounds.right = bounds.right.map_or_else(
                            || Some(line_a.coords[0]),
                            |v| {
                                Some(if is_corner {
                                    v.max(line_a.coords[0])
                                } else {
                                    v.min(line_a.coords[0])
                                })
                            },
                        );
                    }
                    Direction::Up => {
                        bounds.top = bounds.top.map_or_else(
                            || Some(line_a.coords[1]),
                            |v| {
                                Some(if is_corner {
                                    v.min(line_a.coords[1])
                                } else {
                                    v.max(line_a.coords[1])
                                })
                            },
                        );
                    }
                    Direction::Down => {
                        bounds.bottom = bounds.bottom.map_or_else(
                            || Some(line_a.coords[1]),
                            |v| {
                                Some(if is_corner {
                                    v.max(line_a.coords[1])
                                } else {
                                    v.min(line_a.coords[1])
                                })
                            },
                        );
                    }
                }
            }

            (absolute_bounds, corner_bounds)
        },
    );

    VisibilityBounds {
        left: absolute_bounds.left.or(corner_bounds.left),
        right: absolute_bounds.right.or(corner_bounds.right),
        top: absolute_bounds.top.or(corner_bounds.top),
        bottom: absolute_bounds.bottom.or(corner_bounds.bottom),
    }
}

/// Builds visibility bounds for a list of [`IndexedCoords`] based on a polygon.
pub fn build_visibility_bounds_for_indexed_coords(
    mut indexed_coords: Vec<IndexedCoords>,
) -> Vec<IndexedCoords> {
    let bounds = indexed_coords
        .iter()
        .map(|indexed_coord| find_visibility_bounds(&indexed_coord.coords, &indexed_coords))
        .collect::<Vec<_>>();

    indexed_coords
        .iter_mut()
        .zip(bounds)
        .for_each(|(indexed_coord, visibility_bounds)| {
            indexed_coord.visibility_bounds = Some(visibility_bounds);
        });

    indexed_coords
}

#[cfg(test)]
mod test_blocks_visibility {
    use super::*;

    macro_rules! create_test {
        ($name:ident(from=$from:expr, line=$line:expr) = $expected:expr) => {
            #[test]
            fn $name() {
                let coords = $line;
                let line = [
                    &IndexedCoords::new(0, coords[0]),
                    &IndexedCoords::new(1, coords[1]),
                ];

                assert_eq!(blocks_visibility(&$from, line), $expected);
            }
        };
    }

    create_test!(
        blocked_left(from = [5, 3], line = [[3, 1], [3, 5]]) = Some((Direction::Left, false))
    );
    create_test!(
        blocked_right(from = [1, 3], line = [[3, 1], [3, 5]]) = Some((Direction::Right, false))
    );
    create_test!(blocked_up(from = [3, 5], line = [[1, 3], [5, 3]]) = Some((Direction::Up, false)));
    create_test!(
        blocked_down(from = [3, 1], line = [[1, 3], [5, 3]]) = Some((Direction::Down, false))
    );
    create_test!(horizontal_out_of_bounds_right(from = [6, 2], line = [[1, 3], [5, 3]]) = None);
    create_test!(horizontal_out_of_bounds_left(from = [0, 2], line = [[1, 3], [5, 3]]) = None);
    create_test!(vertical_out_of_bounds_up(from = [2, 6], line = [[3, 1], [3, 5]]) = None);
    create_test!(vertical_out_of_bounds_down(from = [2, 0], line = [[3, 1], [3, 5]]) = None);
    create_test!(horizontal_on_the_line(from = [1, 3], line = [[1, 3], [5, 3]]) = None);
    create_test!(vertical_on_the_line(from = [3, 3], line = [[3, 1], [3, 5]]) = None);
    create_test!(
        horizontal_hit_left_edge(from = [1, 2], line = [[1, 3], [5, 3]]) =
            Some((Direction::Down, true))
    );
    create_test!(
        horizontal_hit_right_edge(from = [5, 4], line = [[1, 3], [5, 3]]) =
            Some((Direction::Up, true))
    );
    create_test!(
        vertical_hit_top_edge(from = [2, 5], line = [[3, 1], [3, 5]]) =
            Some((Direction::Right, true))
    );
    create_test!(
        vertical_hit_bottom_edge(from = [4, 1], line = [[3, 1], [3, 5]]) =
            Some((Direction::Left, true))
    );
}

#[cfg(test)]
mod test_find_visibility_bounds {
    use super::*;

    mod simple_square {
        use super::*;

        const POLYGON: [IndexedCoords; 5] = [
            IndexedCoords::new(0, [1, 1]),
            IndexedCoords::new(1, [1, 5]),
            IndexedCoords::new(2, [5, 5]),
            IndexedCoords::new(3, [5, 1]),
            IndexedCoords::new(4, [1, 1]),
        ];

        macro_rules! create_test {
            ($name:ident($from:expr) = [$left:expr, $right:expr, $top:expr, $bottom:expr]) => {
                #[test]
                fn $name() {
                    let from = $from;
                    let bounds = find_visibility_bounds(&from, &POLYGON);

                    assert_eq!(
                        bounds,
                        VisibilityBounds {
                            left: $left,
                            right: $right,
                            top: $top,
                            bottom: $bottom
                        }
                    );
                }
            };
        }

        create_test!(center([3, 3]) = [Some(1), Some(5), Some(1), Some(5)]);
        create_test!(left_edge([1, 3]) = [None, Some(5), Some(1), Some(5)]);
        create_test!(right_edge([5, 3]) = [Some(1), None, Some(1), Some(5)]);
        create_test!(top_edge([3, 1]) = [Some(1), Some(5), None, Some(5)]);
        create_test!(bottom_edge([3, 5]) = [Some(1), Some(5), Some(1), None]);
        create_test!(top_left_corner([1, 1]) = [None, Some(5), None, Some(5)]);
        create_test!(top_right_corner([5, 1]) = [Some(1), None, None, Some(5)]);
        create_test!(bottom_left_corner([1, 5]) = [None, Some(5), Some(1), None]);
        create_test!(bottom_right_corner([5, 5]) = [Some(1), None, Some(1), None]);
        create_test!(outside_horizontal([6, 3]) = [Some(5), None, None, None]);
        create_test!(outside_vertical([3, 0]) = [None, None, None, Some(1)]);
    }

    mod concave {
        use super::*;

        const POLYGON: [IndexedCoords; 8] = [
            IndexedCoords::new(0, [1, 1]),
            IndexedCoords::new(1, [5, 1]),
            IndexedCoords::new(2, [5, 3]),
            IndexedCoords::new(3, [3, 3]),
            IndexedCoords::new(4, [3, 5]),
            IndexedCoords::new(5, [5, 5]),
            IndexedCoords::new(6, [5, 7]),
            IndexedCoords::new(7, [1, 7]),
        ];

        macro_rules! create_test {
            ($name:ident($from:expr) = [$left:expr, $right:expr, $top:expr, $bottom:expr]) => {
                #[test]
                fn $name() {
                    let from = $from;
                    let bounds = find_visibility_bounds(&from, &POLYGON);

                    assert_eq!(
                        bounds,
                        VisibilityBounds {
                            left: $left,
                            right: $right,
                            top: $top,
                            bottom: $bottom
                        }
                    );
                }
            };
        }

        create_test!(inside_polygon([2, 4]) = [Some(1), Some(3), Some(1), Some(7)]);
        create_test!(in_concavity([4, 4]) = [Some(3), None, Some(3), Some(5)]);
        create_test!(outside_right([6, 4]) = [Some(3), None, None, None]);
        // create_test!(outside_lower_right([6, 7]) = [Some(5), None, None, None]);
        create_test!(in_lower_jaw([4, 6]) = [Some(1), Some(5), Some(5), Some(7)]);
        create_test!(on_lower_jaw_edge([4, 5]) = [Some(1), Some(5), Some(3), Some(7)]);
    }

    mod example {
        use super::*;
        use crate::indexed_coords_from_text;
        const INPUT: &str = "7,1
                             11,1
                             11,7
                             9,7
                             9,5
                             2,5
                             2,3
                             7,3";

        macro_rules! create_test {
            ($name:ident($from:expr) = [$left:expr, $right:expr, $top:expr, $bottom:expr]) => {
                #[test]
                fn $name() {
                    let polygon =
                        indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");
                    let from = $from;
                    let bounds = find_visibility_bounds(&from, &polygon);

                    assert_eq!(
                        bounds,
                        VisibilityBounds {
                            left: $left,
                            right: $right,
                            top: $top,
                            bottom: $bottom
                        }
                    );
                }
            };
        }

        create_test!(inside([5, 5]) = [Some(2), Some(11), Some(3), None]);
        // This will fail because of the visibility limitation described in the module docstring
        create_test!(corner([2, 3]) = [None, Some(11), None, Some(5)]);
    }
}
