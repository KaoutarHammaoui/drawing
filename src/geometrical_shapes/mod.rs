use rand::Rng;
use raster::Color;

mod point;
mod traits;
mod line;
mod rectangle;
mod triangle;
mod circle;

pub use point::Point;
pub use traits::{Displayable,Drawable};
pub use line::Line;
pub use circle::Circle;
pub use rectangle::Rectangle;


pub(super) fn random_color()->Color {
    let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
            rng.gen_range(0..=225),
        )
}