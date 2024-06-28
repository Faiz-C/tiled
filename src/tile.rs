use crate::geometry::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub app_process_id: i32,
    pub boundary: Rect
}

impl Tile {
    pub fn new(app_process_id: i32, boundary: Rect) -> Tile {
        return Tile { app_process_id, boundary }
    }
}
