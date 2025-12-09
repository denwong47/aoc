use crate::TessellationFillError;

/// Cell state representation for tessellation fill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CellState {
    Unfilled = 0_u8,
    Corners = 1_u8,
    Edges = 2_u8,
    FilledA = 3_u8,
    FilledB = 4_u8,
    FilledC = 5_u8,
    FilledD = 6_u8,
    FilledE = 7_u8,
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => CellState::Unfilled,
            1 => CellState::Corners,
            2 => CellState::Edges,
            3 => CellState::FilledA,
            4 => CellState::FilledB,
            5 => CellState::FilledC,
            6 => CellState::FilledD,
            7 => CellState::FilledE,
            _ => CellState::Unfilled, // Default case
        }
    }
}

impl CellState {
    pub fn is_border(&self) -> bool {
        matches!(self, CellState::Corners | CellState::Edges)
    }
    pub fn is_filled(&self) -> bool {
        matches!(
            self,
            CellState::FilledA
                | CellState::FilledB
                | CellState::FilledC
                | CellState::FilledD
                | CellState::FilledE
        )
    }
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Unfilled
    }
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            CellState::Unfilled => "\x1b[37m.\x1b[0m",
            CellState::Corners => "\x1b[31m#\x1b[0m",
            CellState::Edges => "\x1b[31mX\x1b[0m",
            CellState::FilledA => "\x1b[32mA\x1b[0m",
            CellState::FilledB => "\x1b[33mB\x1b[0m",
            CellState::FilledC => "\x1b[34mC\x1b[0m",
            CellState::FilledD => "\x1b[35mD\x1b[0m",
            CellState::FilledE => "\x1b[36mE\x1b[0m",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FillState {
    A = 3_u8,
    B = 4_u8,
    C = 5_u8,
    D = 6_u8,
    E = 7_u8,
}

impl From<FillState> for CellState {
    fn from(fill_state: FillState) -> Self {
        match fill_state {
            FillState::A => CellState::FilledA,
            FillState::B => CellState::FilledB,
            FillState::C => CellState::FilledC,
            FillState::D => CellState::FilledD,
            FillState::E => CellState::FilledE,
        }
    }
}

impl TryFrom<CellState> for FillState {
    type Error = TessellationFillError;

    fn try_from(cell_state: CellState) -> Result<Self, Self::Error> {
        match cell_state {
            CellState::FilledA => Ok(FillState::A),
            CellState::FilledB => Ok(FillState::B),
            CellState::FilledC => Ok(FillState::C),
            CellState::FilledD => Ok(FillState::D),
            _ => Err(TessellationFillError::NotAFillState(cell_state)),
        }
    }
}

pub type Unit = i32;
pub type Dimension = u32;
pub type Coords = (Unit, Unit);
pub type Rectangle = (Coords, Coords); // ((x0, y0), (x1, y1))

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self) -> Coords {
        match self {
            // This follows the AOC 2025 Day 9 convention - y increases downwards
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
