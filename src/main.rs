use crate::geometry::rectangle::Rect;

mod geometry;
mod tiles;
mod tree;

fn main() {

    let rect = Rect::new(0, 0, 1920, 1080);

    println!("We made a rectangle: {:?}", rect);
}
