use super::{Displayable, Drawable};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// Implement the Displayable trait for Image.
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    // A random point inside the image.
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point::new(rng.gen_range(0..width), rng.gen_range(0..height))
    }
}

// Implement the Drawable trait for Point.
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0) // red
    }
}
