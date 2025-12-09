use crate::models::{Dimension, Grid, Unit};

pub struct Slice<'g> {
    grid: &'g Grid,
    x_start: Unit,
    y_start: Unit,
    width: Dimension,
    height: Dimension,
}
