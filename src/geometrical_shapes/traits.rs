use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, img:&mut Image);
    fn color(&self) -> Color{
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
        )
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}