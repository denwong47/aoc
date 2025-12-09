use thiserror::Error;

use crate::models::{CellState, Coords};

#[derive(Error, Debug)]
pub enum TessellationFillError {
    #[error("Coordinates {coords:?} are out of bounds for grid of size {width}x{height}.")]
    CoordsOutOfBounds {
        coords: Coords,
        width: u32,
        height: u32,
    },

    #[error("CellState::{0:?} is not a valid fill state.")]
    NotAFillState(CellState),

    #[error("Atomic condition not met when trying to set cell at {0:?}.")]
    AtomicConditionNotMet(Coords),
}
