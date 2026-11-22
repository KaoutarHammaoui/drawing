use rand::Rng;
use raster::Color;

mod circle;
mod line;
mod point;
mod rectangle;
mod traits;
mod triangle;

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;
pub use traits::{Displayable, Drawable};
pub use triangle::Triangle;

#[cfg(test)]
mod tests;

pub(super) fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(
        rng.gen_range(0..=225),
        rng.gen_range(0..=225),
        rng.gen_range(0..=225),
    )
}
