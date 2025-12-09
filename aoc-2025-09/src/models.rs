pub type Coord = u32;
pub type Coords = [Coord; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct VisibilityBounds {
    pub left: Option<Coord>,
    pub right: Option<Coord>,
    pub top: Option<Coord>,
    pub bottom: Option<Coord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IndexedCoords {
    pub index: usize,
    pub coords: Coords,
    pub visibility_bounds: Option<VisibilityBounds>,
}

impl IndexedCoords {
    pub const fn new(index: usize, coords: Coords) -> Self {
        Self {
            index,
            coords,
            visibility_bounds: None,
        }
    }

    pub const fn from_coords(coords: Coords) -> Self {
        Self {
            index: 0,
            coords,
            visibility_bounds: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub bounding: (Coord, Coord, Coord, Coord), // (min_x, max_x, min_y, max_y)
    pub original_coords: [IndexedCoords; 2],
}

impl Rectangle {
    pub fn new(point_a: IndexedCoords, point_b: IndexedCoords) -> Self {
        // Ensure consistent ordering of coordinates
        let original_coords = if point_a.index < point_b.index {
            [point_a, point_b]
        } else {
            [point_b, point_a]
        };

        let bounding = (
            original_coords[0].coords[0].min(original_coords[1].coords[0]),
            original_coords[0].coords[0].max(original_coords[1].coords[0]),
            original_coords[0].coords[1].min(original_coords[1].coords[1]),
            original_coords[0].coords[1].max(original_coords[1].coords[1]),
        );

        Self {
            bounding,
            original_coords,
        }
    }

    pub fn top_left(&self) -> Coords {
        [self.bounding.0, self.bounding.2]
    }

    pub fn top_right(&self) -> Coords {
        [self.bounding.1, self.bounding.2]
    }

    pub fn bottom_left(&self) -> Coords {
        [self.bounding.0, self.bounding.3]
    }

    pub fn bottom_right(&self) -> Coords {
        [self.bounding.1, self.bounding.3]
    }

    /// Returns an iterator over the original points associated with each corner of the rectangle.
    ///
    /// This current implementation is horribly inefficient (O(n)) but n is always 2 and there are only
    /// 4 corners, so... ¯\_(ツ)_/¯
    pub fn original_points_by_corners(&self) -> impl Iterator<Item = (Corner, IndexedCoords)> + '_ {
        self.original_coords.iter().map(|ic| match ic.coords {
            coords if coords == self.top_left() => (Corner::TopLeft, *ic),
            coords if coords == self.top_right() => (Corner::TopRight, *ic),
            coords if coords == self.bottom_left() => (Corner::BottomLeft, *ic),
            coords if coords == self.bottom_right() => (Corner::BottomRight, *ic),
            _ => panic!(
                "Unreacahable: Original coordinate {:?} does not match any rectangle corner",
                ic.coords
            ),
        })
    }

    /// The way area is calculated is a bit unusual - it says that `2,3` to `7,3` is width 1,
    /// not 6.
    pub fn width(&self) -> u32 {
        self.bounding.1 - self.bounding.0 + 1
    }

    /// The way area is calculated is a bit unusual - it says that `2,3` to `7,3` is height 1,
    /// not 0.
    pub fn height(&self) -> u32 {
        self.bounding.3 - self.bounding.2 + 1
    }

    pub fn area(&self) -> u64 {
        self.width() as u64 * self.height() as u64
    }

    pub fn compare(
        &self,
        other: &Self,
        predicate: impl Fn(&Self, &Self) -> anyhow::Result<std::cmp::Ordering>,
    ) -> anyhow::Result<std::cmp::Ordering> {
        predicate(self, other)
    }
}
