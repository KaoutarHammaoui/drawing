use crate::geometrical_shapes::random_color;

use super::{Displayable, Drawable, Point};
use rand::Rng;
use raster::{Image,Color};

pub struct Circle {
    pub(super) center: Point,
    pub(super) raduis: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, raduis: i32) -> Self {
        let color=random_color();
        Self {
            center: center.clone(),
            raduis,
            color,
        }
    }

    pub fn random(w: i32, h: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(w, h);
        let raduis = rng.gen_range(1..=50);
        Self::new(&center, raduis)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let (cx, cy) = (self.center.x, self.center.y);
        let (mut x, mut y) = (0, self.raduis);
        let mut p = 1 - self.raduis;

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
