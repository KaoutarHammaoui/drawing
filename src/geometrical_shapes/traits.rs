use rand::Rng;
use raster::{Color, Image};

// Anything that can be drawn on an image.
pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
        )
    }
}

// Anything that can show one pixel.
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
