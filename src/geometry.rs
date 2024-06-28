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
    pub bottom_right: Point
}

impl Rect {

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        return Rect {
            top_left: Point { x, y },
            bottom_right: Point { x: x + width, y: y + height }
        }
    }

    pub fn from_dimensions(width: i32, height: i32) -> Rect {
        return Self::new(0, 0, width, height)
    }

    pub fn width(&self) -> i32 {
        return self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> i32 {
        return self.bottom_right.y - self.top_left.y
    }

    pub fn split_horizontally(&self) -> (Rect, Rect) {
        let height = self.bottom_right.y - self.top_left.y;
        let half_height = height / 2;

        return (
            Rect::new(self.top_left.x, self.top_left.y, self.width(), half_height),
            Rect::new(self.top_left.x, half_height, self.width(), half_height),
        )
    }

    pub fn split_vertically(&self) -> (Rect, Rect) {
        let width = self.bottom_right.x - self.top_left.x;
        let half_width = width / 2;

        return (
            Rect::new(self.top_left.x, self.top_left.y, half_width, self.height()),
            Rect::new(half_width, self.top_left.y, half_width, self.height()),
        )
    }

}
