use super::{Drawable,Point};
use raster::{Color,Image};

pub struct Circle{
    center:Point,
    raduis:i32,
}


impl Circle{
    pub fn new(center:&Point,raduis:i32)->Self{
        Self{
            center:center.clone(),
            raduis,
        }
    }
}