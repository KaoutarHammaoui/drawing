mod circle;
mod line;
mod point;
mod rectangle;
mod traits;
mod triangle;

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use traits::{Displayable, Drawable};

#[cfg(test)]
mod tests;
