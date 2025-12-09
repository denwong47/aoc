use crate::{
    TessellationFillError,
    models::{CellState, Coords, Dimension, Direction, FillState, Rectangle, Unit},
};
use itertools::Itertools;

/// Trait for types that can represent a 2D grid of CellStates.
pub trait Is2DCellStates {
    fn height(&self) -> Dimension;
    fn width(&self) -> Dimension;
    fn get(&self, coords: Coords) -> Option<CellState>;
    fn set(&self, coords: Coords, state: CellState) -> Result<bool, TessellationFillError> {
        self.set_if(coords, state, None)
    }
    fn set_if(
        &self,
        coords: Coords,
        state: CellState,
        condition: Option<CellState>,
    ) -> Result<bool, TessellationFillError>;
    fn find(&self, predicate: impl Fn(&CellState) -> bool) -> impl Iterator<Item = Coords> {
        let width = self.width() as Unit;
        let height = self.height() as Unit;

        (0..width)
            .cartesian_product(0..height)
            .filter_map(move |(x, y)| {
                let coords = (x, y);
                match self.get(coords) {
                    Some(cell_state) if predicate(&cell_state) => Some(coords),
                    // Unreachable - just continue
                    _ => None,
                }
            })
    }
    fn are_all(&self, predicate: impl Fn(&CellState) -> bool) -> bool {
        let width = self.width() as Unit;
        let height = self.height() as Unit;

        (0..width).cartesian_product(0..height).all(|(x, y)| {
            let coords = (x, y);
            match self.get(coords) {
                Some(cell_state) => predicate(&cell_state),
                // Unreachable - treat as false
                None => false,
            }
        })
    }
    fn has(&self, predicate: impl Fn(&CellState) -> bool) -> bool {
        self.find(predicate).next().is_some()
    }
    fn is_empty(&self) -> bool {
        self.are_all(|state| *state == CellState::Unfilled)
    }
    fn edge_is(&self, direction: Direction, predicate: impl Fn(&CellState) -> bool) -> bool {
        let width = self.width() as Unit;
        let height = self.height() as Unit;

        let mut edge_coords_iter: Box<dyn Iterator<Item = Coords>> = match direction {
            Direction::Up => Box::new((0..width).map(|x| (x, 0))),
            Direction::Down => Box::new((0..width).map(|x| (x, height - 1))),
            Direction::Left => Box::new((0..height).map(|y| (0, y))),
            Direction::Right => Box::new((0..height).map(|y| (width - 1, y))),
        };

        edge_coords_iter.all(|coords| match self.get(coords) {
            Some(cell_state) => predicate(&cell_state),
            // Unreachable - treat as false
            None => false,
        })
    }
    fn edge_has(&self, direction: Direction, predicate: impl Fn(&CellState) -> bool) -> bool {
        let width = self.width() as Unit;
        let height = self.height() as Unit;

        let mut edge_coords_iter: Box<dyn Iterator<Item = Coords>> = match direction {
            Direction::Up => Box::new((0..width).map(|x| (x, 0))),
            Direction::Down => Box::new((0..width).map(|x| (x, height - 1))),
            Direction::Left => Box::new((0..height).map(|y| (0, y))),
            Direction::Right => Box::new((0..height).map(|y| (width - 1, y))),
        };

        edge_coords_iter.any(|coords| match self.get(coords) {
            Some(cell_state) => predicate(&cell_state),
            // Unreachable - treat as false
            None => false,
        })
    }
    fn fill_rectangle(&self, rectangle: &Rectangle) -> bool;
    fn fill_from(&self, origin: Coords, fill: FillState) -> bool;
    fn display_to(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        let width = self.width() as Unit;
        let height = self.height() as Unit;
        for x in 0..width {
            write!(f, "{}", x % 10)?;
        }
        writeln!(f)?;
        for y in 0..height {
            write!(f, "{} ", y % 10)?;
            for x in 0..width {
                let colour = self.get((x, y)).unwrap_or(CellState::Unfilled);
                write!(f, "{}", colour)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
