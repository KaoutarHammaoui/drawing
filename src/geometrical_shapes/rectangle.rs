use crate::geometrical_shapes::{random_color, Line, Point};

use super::{ Drawable};

use raster::{Color, Image};

pub struct Rectangle{
    pub a:Point,
    pub b:Point,
    pub color:Color,
}

impl Rectangle {
    pub fn new(a:&Point,b:&Point)-> Self{
        let min_x=(a.x).min(b.x);
        let max_x=(a.x).max(b.x);
        let min_y=(a.y).min(b.y);
        let max_y=(a.y).max(b.y);
        
        Rectangle { 
            a: Point::new(min_x,min_y), 
            b: Point::new(max_x,max_y),
            color: random_color() 
        }
        
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let c= &Point{x: self.b.x, y: self.a.y };//top-right
        let d= &Point{x: self.a.x, y: self.b.y };// bottom-left
        let color=&self.color;

        let top=Line::with_color(&self.a, c,color);

        let bottom=Line::with_color(d, &self.b, color);
        let left=Line::with_color(&self.a, d, color);
        let right=Line::with_color(c, &self.b, color);
        
        
        top.draw(image);
        right.draw(image);
        bottom.draw(image);
        left.draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

