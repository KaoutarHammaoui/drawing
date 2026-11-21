use super::{Drawable,Point,Displayable};
use raster::{Color,Image};
use rand::Rng;

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

    pub fn random(w:i32,h:i32)->Self{
        let mut rng= rand::thread_rng();
        let center=Point::random(w,h);
        let raduis=rng.gen_range(1..=50);
        Self::new(&center, raduis)
    }
}


impl Drawable for Circle{

    fn draw(&self,image:&mut Image){
        let color=self.color();
        let (cx,cy)= (self.center.x,self.center.y);
        let (mut x,mut y)=(0,self.raduis);
        let mut p=1-self.raduis;

        while x<=y{
            for(dx,dy) in [
                (x,y),(-x,y),(x,-y),(-x,-y),
                (y, x), (-y, x), (y, -x), (-y, -x),
            ]{
                image.display(cx+dx,cy+dy,color.clone());
            }
            x+=1;
            if p<0{
                p+=2*x+1;
            }else{
                y-=1;
                p+=2 *(x-y)+1;
            }
        }
    }
}


#[test]
fn zero_radius_draws_one_pixel() {
    let mut image = Image::blank(10, 10);
    let circle = Circle::new(&Point::new(5, 5), 0);

    circle.draw(&mut image);

    assert_eq!(image.get_pixel(5, 5).unwrap().g, 255);
}