use super::{Displayable, Drawable, random_color};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: x.abs(),
            y: y.abs(),
            color: random_color(),
        }
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
        self.color.clone()
    }
}
