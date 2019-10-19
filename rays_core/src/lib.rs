pub mod util;

pub mod tuples;
pub mod color;
pub mod canvas;
pub mod image;
pub mod matrices;

pub use canvas::Canvas;
pub use color::Color;

#[cfg(test)]
mod tests {
}
