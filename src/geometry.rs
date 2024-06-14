#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point {
            x,
            y
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
