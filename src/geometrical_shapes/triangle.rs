use super::{Drawable, Line, Point};
use raster::Image;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(&self.a, &self.b).draw(image);
        Line::new(&self.b, &self.c).draw(image);
        Line::new(&self.c, &self.a).draw(image);
    }
}