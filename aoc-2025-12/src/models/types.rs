use bitvec::{bitbox, boxed::BitBox};
use bitvec_simd::BitVec;

pub type StateStorage = BitVec;
pub type PlacementMask = BitBox;

pub const SHAPE_WIDTH: usize = 3;
pub const SHAPE_HEIGHT: usize = 3;
pub type InnerShape = [bool; SHAPE_WIDTH * SHAPE_HEIGHT];

/// This may not be followed - Shape has its own display logic
pub const FILLED_DISPLAY: char = '█';
pub const EMPTY_DISPLAY: char = '░';

pub fn build_new_placement_mask(placements_len: usize) -> PlacementMask {
    bitbox![1; placements_len]
}
