use crate::geometrical_shapes::random_color;

use super::{Displayable, Drawable, Point};
use rand::Rng;
use raster::{Color, Image};

pub struct Circle {
    pub(super) center: Point,
    pub(super) radius: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        let color = random_color();
        Self {
            center: center.clone(),
            radius,
            color,
        }
    }

    pub fn random(w: i32, h: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(w, h);
        let radius = rng.gen_range(10..=500);
        Self::new(&center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let (cx, cy) = (self.center.x, self.center.y);
        let (mut x, mut y) = (0, self.radius);
        let mut p = 1 - self.radius;

        while x <= y {
            for (dx, dy) in [
                (x, y),
                (-x, y),
                (x, -y),
                (-x, -y),
                (y, x),
                (-y, x),
                (y, -x),
                (-y, -x),
            ] {
                image.display(cx + dx, cy + dy, color.clone());
            }
            x += 1;
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y -= 1;
                p += 2 * (x - y) + 1;
            }
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
