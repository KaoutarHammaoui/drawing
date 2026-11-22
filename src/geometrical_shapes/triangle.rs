use crate::geometrical_shapes::random_color;

use super::{Drawable, Line, Point};
use raster::{Color, Image};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        let color = random_color();
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            color,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = &self.color;

        // Mirror negative coordinates so the triangle stays inside the image,
        
        let a = Point::new(self.a.x.abs(), self.a.y.abs());
        let b = Point::new(self.b.x.abs(), self.b.y.abs());
        let c = Point::new(self.c.x.abs(), self.c.y.abs());

        Line::with_color(&a, &b, color).draw(image);
        Line::with_color(&b, &c, color).draw(image);
        Line::with_color(&c, &a, color).draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
