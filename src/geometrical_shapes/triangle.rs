use crate::geometrical_shapes::random_color;

use super::{Drawable, Line, Point};
use raster::{Image,Color};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        let color=random_color();
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
        let color=&self.color;

        Line::with_color(&self.a, &self.b,color).draw(image);
        Line::with_color(&self.b, &self.c,color).draw(image);
        Line::with_color(&self.c, &self.a,color).draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}