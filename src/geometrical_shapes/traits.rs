use raster::{Color, Image};

// Anything that can be drawn on an image.
pub trait Drawable {
    fn draw(&self, img:&mut Image);
    fn color(&self) -> Color;
}

// Anything that can show one pixel.
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
