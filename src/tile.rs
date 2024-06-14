use crate::geometry::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    boundary: Rect,
    // Insert application window details here
}

impl Tile {
    pub fn new(boundary: Rect) -> Tile {
        return Tile { boundary }
    }
}
