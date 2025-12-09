use crate::models::{Coords, Rectangle};
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;

#[cfg(feature = "profile")]
use std::time::{Duration, Instant};

#[cfg(feature = "profile")]
const LOG_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Red = 1,
    Green = 2,
    White = 3,
    Yellow = 4,
    Colourless = 0,
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Red => write!(f, "\x1b[31m#\x1b[0m"),
            Colour::Green => write!(f, "\x1b[32mX\x1b[0m"),
            Colour::White => write!(f, "\x1b[37m.\x1b[0m"),
            Colour::Yellow => write!(f, "\x1b[33mO\x1b[0m"),
            Colour::Colourless => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Colour>,
}

impl Grid {
    pub fn new(width: u32, height: u32, colour: Colour) -> Self {
        Self {
            width,
            height,
            cells: vec![colour; width as usize * height as usize],
        }
    }

    pub fn new_to_fit<'a>(coords: impl Iterator<Item = &'a Coords>, colour: Colour) -> Self {
        let (max_x, max_y) = coords.fold((0, 0), |(max_x, max_y), &coord| {
            (max_x.max(coord[0]), max_y.max(coord[1]))
        });

        Self::new(max_x + 2, max_y + 2, colour)
    }

    pub fn set(&mut self, x: u32, y: u32, colour: Colour) {
        if x < self.width && y < self.height {
            self.cells[y as usize * self.width as usize + x as usize] = colour;
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Colour> {
        if x < self.width && y < self.height {
            Some(self.cells[y as usize * self.width as usize + x as usize])
        } else {
            None
        }
    }

    pub fn draw_rectangle_if(
        &mut self,
        rect: &Rectangle,
        predicate: impl Fn(Option<Colour>) -> bool,
        colour: Colour,
    ) -> anyhow::Result<()> {
        let (x0, x1, y0, y1) = rect.bounding;

        (x0..=x1).cartesian_product(y0..=y1).try_for_each(|(x, y)| {
            if predicate(self.get(x, y)) {
                self.set(x, y, colour);
                Ok(())
            } else {
                anyhow::bail!(
                    "Predicate failed for cell ({}, {}) with colour {:?}",
                    x,
                    y,
                    self.get(x, y)
                )
            }
        })
    }

    /// Recursive helper for flood fill.
    pub fn fill_from(&mut self, x: u32, y: u32, colour: Colour) {
        #[cfg(feature = "profile")]
        let mut last_log = Instant::now();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            #[cfg(feature = "profile")]
            {
                let now = Instant::now();
                if now.duration_since(last_log) >= LOG_INTERVAL {
                    eprintln!(
                        "Flood fill progress: filled {} nodes so far, {} to go in queue...",
                        seen.len(),
                        queue.len()
                    );
                    last_log = now;
                }
            }
            #[cfg(feature = "trace")]
            {
                eprintln!("Filling at ({}, {})", x, y);
            }
            match self.get(x, y) {
                // If the cell is already filled with a different colour, stop
                Some(existing_colour) if existing_colour != Colour::Colourless => (),
                // If the cell is uncoloured, fill it and continue flood fill
                Some(_) => {
                    self.set(x, y, colour);

                    for (dx, dy) in (-1..=1)
                        .cartesian_product(-1..=1)
                        // Only orthogonal directions, not diagonal
                        .filter(|(dx, dy)| (*dx == 0) ^ (*dy == 0))
                    {
                        let shifted_x = (x as i32).checked_add(dx);
                        let shifted_y = (y as i32).checked_add(dy);

                        if let (Some(sx), Some(sy)) = (shifted_x, shifted_y) {
                            // If the cell is uncoloured, fill it and continue flood fill
                            // from there
                            if self.get(sx as u32, sy as u32) == Some(Colour::Colourless) {
                                if seen.insert((sx as u32, sy as u32)) {
                                    queue.push_back((sx as u32, sy as u32));
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }

        #[cfg(feature = "trace")]
        {
            eprintln!("Filled from {} nodes with colour {:?}", seen.len(), colour);
        }
    }

    /// Draw a boundary defined by an iterator of coordinates.
    ///
    /// The coordinates must be orthogonally linked, i.e., each coordinates
    /// must only differ by 1 in either the x or y axis from the previous coordinate.
    ///
    /// Otherwise a block will be drawn between non-adjacent coordinates.
    pub fn boundary(&mut self, coords: &[Coords]) {
        for (node_a, node_b) in coords.iter().circular_tuple_windows() {
            let range_x = if node_a[0] <= node_b[0] {
                node_a[0]..=node_b[0]
            } else {
                node_b[0]..=node_a[0]
            };

            let range_y = if node_a[1] <= node_b[1] {
                node_a[1]..=node_b[1]
            } else {
                node_b[1]..=node_a[1]
            };

            for x in range_x.clone() {
                for y in range_y.clone() {
                    self.set(x, y, Colour::Green);
                }
            }

            self.set(node_a[0], node_a[1], Colour::Red);
            self.set(node_b[0], node_b[1], Colour::Red);
        }
    }

    /// Check that all cells along the line from `start` to `end` satisfy the given predicate.
    pub fn check_area(
        &self,
        start: &Coords,
        end: &Coords,
        predicate: impl Fn(Option<Colour>) -> bool,
    ) -> bool {
        let range_x = if start[0] <= end[0] {
            start[0]..=end[0]
        } else {
            end[0]..=start[0]
        };
        let range_y = if start[1] <= end[1] {
            start[1]..=end[1]
        } else {
            end[1]..=start[1]
        };

        range_x
            .cartesian_product(range_y)
            .all(|(x, y)| predicate(self.get(x, y)))
    }

    pub fn check_rectangle_border(
        &self,
        rect: &Rectangle,
        predicate: impl Fn(Option<Colour>) -> bool,
    ) -> bool {
        let (x0, x1, y0, y1) = rect.bounding;

        self.check_area(&[x0, y0], &[x1, y0], &predicate)
            && self.check_area(&[x0, y1], &[x1, y1], &predicate)
            && self.check_area(&[x0, y0], &[x0, y1], &predicate)
            && self.check_area(&[x1, y0], &[x1, y1], &predicate)
    }

    pub fn colour_count(&self, colour: Colour) -> usize {
        self.cells.iter().filter(|&&c| c == colour).count()
    }

    pub fn save_to(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let colour = self.get(x, y).unwrap_or(Colour::Colourless);
                write!(file, "{}", colour as u8)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }

    pub fn load_from(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len() as u32;
        let width = lines.first().map_or(0, |line| line.len()) as u32;

        let cells = lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '1' => Colour::Red,
                        '2' => Colour::Green,
                        '3' => Colour::White,
                        _ => Colour::Colourless,
                    })
                    .collect::<Vec<Colour>>()
            })
            .collect::<Vec<Colour>>();

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for x in 0..self.width {
            write!(f, "{}", x % 10)?;
        }
        writeln!(f)?;
        for y in 0..self.height {
            write!(f, "{} ", y % 10)?;
            for x in 0..self.width {
                let colour = self.get(x, y).unwrap_or(Colour::Colourless);
                write!(f, "{}", colour)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::indexed_coords_from_text;

    mod concave {
        use super::*;
        const INPUT: &str = "1,1
                             5,1
                             5,3
                             3,3
                             3,5
                             5,5
                             5,7
                             1,7";

        #[test]
        fn test_draw_table() {
            let indexed_coords =
                indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");

            let coords: Vec<Coords> = indexed_coords.iter().map(|ic| ic.coords).collect();
            let mut grid = Grid::new_to_fit(coords.iter(), Colour::Colourless);
            grid.boundary(&coords);
            grid.fill_from(2, 2, Colour::Green);

            eprintln!("{}", grid);

            assert_eq!(grid.get(2, 2), Some(Colour::Green));
            assert_eq!(grid.get(0, 0), Some(Colour::Colourless));
            assert_eq!(grid.get(6, 1), Some(Colour::Colourless));
            assert_eq!(grid.get(4, 4), Some(Colour::Colourless));
            assert_eq!(grid.get(3, 6), Some(Colour::Green));
        }

        #[test]
        fn test_fill_outside() {
            let indexed_coords =
                indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");
            let coords: Vec<Coords> = indexed_coords.iter().map(|ic| ic.coords).collect();
            let mut grid = Grid::new_to_fit(coords.iter(), Colour::Colourless);
            grid.boundary(&coords);
            grid.fill_from(0, 0, Colour::White);
            eprintln!("{}", grid);
        }

        #[test]
        fn test_io() {
            let indexed_coords =
                indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");
            let coords: Vec<Coords> = indexed_coords.iter().map(|ic| ic.coords).collect();
            let mut grid = Grid::new_to_fit(coords.iter(), Colour::Colourless);
            grid.boundary(&coords);
            grid.fill_from(2, 2, Colour::Green);

            let path = std::path::Path::new("test_grid_io.txt");
            grid.save_to(path).expect("Failed to save grid");

            let loaded_grid = Grid::load_from(path).expect("Failed to load grid");

            assert_eq!(grid.width, loaded_grid.width);
            assert_eq!(grid.height, loaded_grid.height);
            assert_eq!(grid.cells, loaded_grid.cells);

            // std::fs::remove_file(path).expect("Failed to remove test file");
        }
    }

    mod example {
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
        fn test_draw_table() {
            let indexed_coords =
                indexed_coords_from_text(INPUT).expect("Failed to parse indexed coords");

            let coords: Vec<Coords> = indexed_coords.iter().map(|ic| ic.coords).collect();
            let mut grid = Grid::new_to_fit(coords.iter(), Colour::Colourless);
            grid.boundary(&coords);
            grid.fill_from(8, 2, Colour::Green);

            eprintln!("{}", grid);

            assert_eq!(grid.get(8, 2), Some(Colour::Green));
            assert_eq!(grid.get(0, 0), Some(Colour::Colourless));
            assert_eq!(grid.get(7, 1), Some(Colour::Red));
            assert_eq!(grid.get(12, 1), Some(Colour::Colourless));
            assert_eq!(grid.get(8, 6), Some(Colour::Colourless));
            assert_eq!(grid.get(9, 6), Some(Colour::Green));
        }
    }
}
