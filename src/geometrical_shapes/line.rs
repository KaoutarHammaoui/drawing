use crate::geometrical_shapes::{random_color, Point};

use super::{Displayable, Drawable};

use raster::{Color, Image};

pub struct Line{
    a:Point,
    b:Point,
    color:Color,
}

impl Line {
    pub fn new(a:&Point,b:&Point)-> Self{
        let color=random_color();
        Line { 
            a:Point::new(a.x, a.y),
            b:Point::new(b.x,b.y),
            color,
        }
    }
    pub(super) fn with_color(a:&Point,b:&Point,color:&Color)->Self{
        Line { 
            a:Point::new(a.x, a.y),
            b:Point::new(b.x,b.y),
            color:color.clone(),
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

        let color=self.color();

        loop{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_same_coordinates() {
        let point = Point::new(3, 3);

        let line = Line::new(&point, &point);

        assert_eq!(line.a.x, 3);
        assert_eq!(line.a.y, 3);
        assert_eq!(line.b.x, 3);
        assert_eq!(line.b.y, 3);
    }

    #[test]
    fn test_random_line_stays_within_bounds() {
        let width = 100;
        let height = 80;

        for _ in 0..1000 {
            let line = Line::random(width, height);

            assert!(line.a.x >= 0 && line.a.x < width);
            assert!(line.a.y >= 0 && line.a.y < height);

            assert!(line.b.x >= 0 && line.b.x < width);
            assert!(line.b.y >= 0 && line.b.y < height);
        }
    }
}
