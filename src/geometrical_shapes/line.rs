use crate::geometrical_shapes::Point;

use super::{Displayable, Drawable};

use raster::{Image};

pub struct Line{
    a:Point,
    b:Point,
}

impl Line {
    pub fn new(a:&Point,b:&Point)-> Self{
        Line { 
            a:Point::new(a.x, a.y),
            b:Point::new(b.x,b.y), 
        }
    }

    pub fn random(width:i32,height:i32) -> Self{
        Line::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color=self.color();

        let dx=(self.b.x-self.a.x).abs();
        let dy=(self.b.y-self.a.y).abs();

        let sx = if self.a.x < self.b.x { 1 } else { -1 };
        let sy = if self.a.y < self.b.y { 1 } else { -1 };

        let mut  x= self.a.x;
        let mut  y= self.a.y;

        let xb=self.b.x;
        let yb=self.b.y;

        let mut error=dx-dy;

        loop{
            image.display(x, y, color.clone());
            if x==xb && y==yb {
                break;
            }

            let e2= 2*error;

            if  e2> -dy {
                error-=dy;
                x+=sx;
            }

            if e2< dx{
                error+=dx;
                y+=sy;
            }
        }


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
    fn test_line_draw_same_coordinates() {
        let point = Point::new(3, 3);
        let line = Line::new(&point, &point);
    
        let mut image = Image::blank(10, 10);
    
        line.draw(&mut image);
    
        assert!(matches!(image.get_pixel(3, 3), Ok(_)));
    }
}
