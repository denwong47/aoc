use std::sync::atomic;

use crate::{
    TessellationFillError,
    models::{CellState, Coords, Dimension, Rectangle, Unit},
    traits,
};

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<atomic::AtomicU8>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|_| atomic::AtomicU8::new(CellState::default() as u8))
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn index_of(&self, coords: Coords) -> Option<usize> {
        let (x, y) = coords;
        if x >= self.width as Unit || y >= self.height as Unit {
            return None;
        }
        Some((y * self.width as Unit + x) as usize)
    }
}

impl traits::Is2DCellStates for Grid {
    fn height(&self) -> Dimension {
        self.height
    }

    fn width(&self) -> Dimension {
        self.width
    }

    fn get(&self, coords: Coords) -> Option<CellState> {
        self.index_of(coords).and_then(|index| {
            let value = self.cells[index].load(atomic::Ordering::Relaxed);
            Some(CellState::from(value))
        })
    }

    fn set_if(
        &self,
        coords: Coords,
        state: CellState,
        condition: Option<CellState>,
    ) -> Result<bool, TessellationFillError> {
        let index =
            self.index_of(coords)
                .ok_or_else(|| TessellationFillError::CoordsOutOfBounds {
                    coords,
                    width: self.width,
                    height: self.height,
                })?;
        let new_value = state as u8;

        match condition {
            Some(cond_state) => {
                let cond_value = cond_state as u8;
                self.cells[index]
                    .compare_exchange(
                        cond_value,
                        new_value,
                        atomic::Ordering::Relaxed,
                        atomic::Ordering::Relaxed,
                    )
                    .map(|_| true)
                    .map_err(|_| TessellationFillError::AtomicConditionNotMet(coords))
            }
            None => {
                self.cells[index].store(new_value, atomic::Ordering::SeqCst);
                Ok(true)
            }
        }
    }

    fn fill_rectangle(&self, rectangle: &Rectangle) -> bool {
        todo!()
    }

    fn fill_from(&self, origin: Coords, fill: super::FillState) -> bool {
        todo!()
    }
}
