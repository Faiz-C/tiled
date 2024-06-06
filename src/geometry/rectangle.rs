use crate::geometry::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top_left: Point,
    pub top_right: Point
}

impl Rect {

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        return Rect {
            top_left: Point { x, y },
            top_right: Point { x: x + width, y: y + height }
        }
    }

}
