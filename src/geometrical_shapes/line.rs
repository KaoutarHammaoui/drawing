use crate::geometrical_shapes::{Point, random_color};

use super::{Displayable, Drawable};

use raster::{Color, Image};

pub struct Line {
    pub a: Point,
    pub b: Point,
    pub color: Color,
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        let color = random_color();
        Line {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            color,
        }
    }
    pub(super) fn with_color(a: &Point, b: &Point, color: &Color) -> Self {
        Line {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            color: color.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.b.x - self.a.x).abs();
        let dy = (self.b.y - self.a.y).abs();

        let sx = if self.a.x < self.b.x { 1 } else { -1 };
        let sy = if self.a.y < self.b.y { 1 } else { -1 };

        let mut x = self.a.x;
        let mut y = self.a.y;

        let xb = self.b.x;
        let yb = self.b.y;

        let mut error = dx - dy;

        let color = self.color();

        loop {
            image.display(x, y, color.clone());
            if x == xb && y == yb {
                break;
            }

            let e2 = 2 * error;

            if e2 > -dy {
                error -= dy;
                x += sx;
            }

            if e2 < dx {
                error += dx;
                y += sy;
            }
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
