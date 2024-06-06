use crate::geometry::rectangle::Rect;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    boundary: Rect,
    // Insert application window details here
}

impl Tile {
    pub fn new(boundary: Rect) -> Tile {
        return Tile { boundary }
    }
}
