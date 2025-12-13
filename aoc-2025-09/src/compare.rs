use crate::models::*;

/// Finds the largest area from a list of indexed coordinates.
///
/// ``indexed_coords`` must be sorted in their index in ascending order.
pub fn find_best_match(
    indexed_coords: &[IndexedCoords],
    predicate: impl Fn(&Rectangle, &Rectangle) -> anyhow::Result<std::cmp::Ordering>,
) -> anyhow::Result<Option<Rectangle>> {
    indexed_coords.iter().fold(
        Ok(None),
        |candidate: anyhow::Result<Option<Rectangle>>, &current| {
            indexed_coords[current.index + 1..]
                .iter()
                .fold(candidate, |inner_candidate, &next| {
                    if let Ok(opt_rec) = inner_candidate {
                        let rect = Rectangle::new(current, next);
                        #[cfg(feature = "trace")]
                        {
                            eprintln!(
                                "Considering rectangle between {:?} and {:?} with area {}",
                                current.coords,
                                next.coords,
                                rect.area()
                            );
                        }
                        match opt_rec {
                            Some(current_candidate) => {
                                if rect.compare(&current_candidate, &predicate)?
                                    == std::cmp::Ordering::Greater
                                {
                                    Ok(Some(rect))
                                } else {
                                    Ok(Some(current_candidate))
                                }
                            }
                            None => Ok(Some(rect)),
                        }
                    } else {
                        inner_candidate
                    }
                })
        },
    )
}

pub fn compare_area_with_visibility(
    candidate: &Rectangle,
    current: &Rectangle,
) -> anyhow::Result<std::cmp::Ordering> {
    let chosen_points = (
        candidate.original_coords[0].index,
        candidate.original_coords[1].index,
    );
    let target = (1, 30);
    match candidate.area().cmp(&current.area()) {
        std::cmp::Ordering::Greater => {
            let results = candidate
                .original_points_by_corners()
                .map(|(corner, indexed_coords)| -> anyhow::Result<bool> {
                    let visbounds = indexed_coords.visibility_bounds.ok_or(anyhow::anyhow!(
                        "Indexed coordinate {:?} has no visibility bounds",
                        indexed_coords.coords
                    ))?;

                    if chosen_points == target{
                        dbg!(&corner, &indexed_coords);
                    }

                    let is_within = match corner {
                        Corner::TopLeft => {
                            // Check if the top left corner can see beyond the candidate rectangle
                            visbounds
                                .right
                                .map(|right_bound| right_bound >= candidate.top_right()[0])
                                .unwrap_or(false)
                                && visbounds
                                    .bottom
                                    .map(|bottom_bound| {
                                        bottom_bound >= candidate.bottom_left()[1]
                                    })
                                    .unwrap_or(false)
                        }
                        Corner::TopRight => {
                            visbounds
                                .left
                                .map(|left_bound| left_bound <= candidate.top_left()[0])
                                .unwrap_or(false)
                                && visbounds
                                    .bottom
                                    .map(|bottom_bound| {
                                        bottom_bound >= candidate.bottom_right()[1]
                                    })
                                    .unwrap_or(false)
                        }
                        Corner::BottomLeft => {
                            visbounds
                                .right
                                .map(|right_bound| right_bound >= candidate.bottom_right()[0])
                                .unwrap_or(false)
                                && visbounds
                                    .top
                                    .map(|top_bound| {
                                        top_bound <= candidate.top_left()[1]
                                    })
                                    .unwrap_or(false)
                        }
                        Corner::BottomRight => {
                            visbounds
                                .left
                                .map(|left_bound| left_bound <= candidate.bottom_left()[0])
                                .unwrap_or(false)
                                && visbounds
                                    .top
                                    .map(|top_bound| top_bound <= candidate.top_right()[1])
                                    .unwrap_or(false)
                        }
                    };

                    if chosen_points == target {
                        dbg!(&is_within);
                    }

                    #[cfg(feature = "trace")]
                    {
                        if is_within {
                            eprintln!(
                                "{:?} at {:?} can see the neighbouring corners of candidate rectangle",
                                corner, indexed_coords.coords
                            );
                        } else {
                            eprintln!(
                                "{:?} at {:?} cannot see the neighbouring corners of candidate rectangle with bounds {:?}",
                                corner, indexed_coords.coords, visbounds
                            );
                        }
                    }

                    Ok(is_within)
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            if results.iter().all(|&v| v) {
                #[cfg(feature = "trace")]
                {
                    eprintln!(
                        "Candidate rectangle with area {} is bigger than current with area {} and is within the polygon",
                        candidate.area(),
                        current.area()
                    );
                }
                Ok(std::cmp::Ordering::Greater)
            } else {
                #[cfg(feature = "trace")]
                {
                    eprintln!(
                        "Candidate rectangle with area {} is bigger than current with area {} but is NOT within the polygon",
                        candidate.area(),
                        current.area()
                    );
                }
                Ok(std::cmp::Ordering::Less)
            }
        }
        ord => Ok(ord),
    }
}

#[cfg(test)]
mod tests_compare_area_with_visibility {
    use crate::{colour, indexed_coords_from_text, models, visibility};

    use super::*;

    macro_rules! create_test {
        ($name:ident($input:expr) = $expected:literal) => {
            #[test]
            fn $name() {
                let indexed_coords = visibility::build_visibility_bounds_for_indexed_coords(
                    indexed_coords_from_text($input).expect("Failed to parse indexed coords"),
                );
                let coords: Vec<models::Coords> =
                    indexed_coords.iter().map(|ic| ic.coords).collect();

                let mut grid = {
                    let mut grid =
                        colour::Grid::new_to_fit(coords.iter(), colour::Colour::Colourless);
                    grid.boundary(&coords);
                    grid.fill_from(0, 0, colour::Colour::White);
                    grid
                };
                eprintln!("Before:\n{}", grid);

                let best_rectangle =
                    find_best_match(&indexed_coords, |a, b| compare_area_with_visibility(a, b))
                        .expect("Error finding best match with visibility")
                        .expect("No rectangle found within polygon");

                let visibility_bounds = best_rectangle
                    .original_points_by_corners()
                    .map(|(c, ic)| (c, ic.visibility_bounds))
                    .collect::<Vec<_>>();

                dbg!(&best_rectangle);
                dbg!(visibility_bounds[0]);
                dbg!(visibility_bounds[1]);

                grid.draw_rectangle_if(
                    &best_rectangle,
                    |colour| colour != Some(colour::Colour::White),
                    colour::Colour::Yellow,
                )
                .expect("Failed to draw rectangle");

                eprintln!("After:\n{}", grid);

                dbg!("Best rectangle: {:?}", &best_rectangle.bounding);
                dbg!(indexed_coords[0].visibility_bounds.as_ref());
                assert_eq!(best_rectangle.area(), $expected);
            }
        };
    }

    const EXAMPLE: &str = "7,1
                           11,1
                           11,7
                           9,7
                           9,5
                           2,5
                           2,3
                           7,3";

    create_test!(example_polygon(EXAMPLE) = 24);

    const VERTICAL_STALAGMITE: &str = "1,3
                                       2,3
                                       2,1
                                       4,1
                                       4,3
                                       11,3
                                       11,8
                                       9,8
                                       9,6
                                       7,6
                                       7,12
                                       5,12
                                       5,7
                                       3,7
                                       3,11
                                       1,11";
    create_test!(v_stalagmite_polygon(VERTICAL_STALAGMITE) = 36);

    const HORIZONTAL_STALAGMITE: &str = "3,1
                                         3,2
                                         1,2
                                         1,4
                                         3,4
                                         3,11
                                         8,11
                                         8,9
                                         6,9
                                         6,7
                                         12,7
                                         12,5
                                         7,5
                                         7,3
                                         11,3
                                         11,1";
    create_test!(h_stalagmite_polygon(HORIZONTAL_STALAGMITE) = 36);

    const PACMAN: &str = "1,6
                          1,5
                          2,5
                          2,4
                          3,4
                          3,3
                          4,3
                          4,2
                          6,2
                          6,1
                          10,1
                          10,2
                          12,2
                          12,3
                          13,3
                          13,4
                          14,4
                          14,12
                          13,12
                          13,13
                          12,13
                          12,14
                          4,14
                          4,13
                          3,13
                          3,12
                          2,12
                          2,11
                          1,11
                          1,10
                          12,10
                          12,6";
    //   0123456789012345
    // 0 ................
    // 1 ......#XXX#.....
    // 2 ....#X#   #X#...
    // 3 ...##       ##..
    // 4 ..##         ##.
    // 5 .##           X.
    // 6 .#XXXXXXXXXX# X.
    // 7 ............X X.
    // 8 ............X X.
    // 9 ............X X.
    // 0 .#XXXXXXXXXX# X.
    // 1 .##           X.
    // 2 ..##         ##.
    // 3 ...##       ##..
    // 4 ....#XXXXXXX#...
    // 5 ................

    // create_test!(pacman(PACMAN) = 36);
}
