use super::{Displayable, Drawable};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point::new(rng.gen_range(0..width), rng.gen_range(0..height))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0) // red
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_stays_inside_image() {
        for _ in 0..1000 {
            let p = Point::random(100, 50);
            assert!(p.x >= 0 && p.x < 100);
            assert!(p.y >= 0 && p.y < 50);
        }
    }

    #[test]
    fn new_keeps_values() {
        let p = Point::new(3, 4);
        assert_eq!((p.x, p.y), (3, 4));
    }

    #[test]
    fn draw_sets_one_pixel() {
        let mut image = Image::blank(10, 10);
        Point::new(5, 5).draw(&mut image);
        assert_eq!(image.get_pixel(5, 5).unwrap().r, 255);
    }
}