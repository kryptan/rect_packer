#[cfg(test)]
extern crate rand;

#[cfg(test)]
extern crate image;

pub use rect::Rect;
pub use packer::Packer;

mod rect;
mod packer;

#[cfg(test)]
mod test;

#[derive(Copy, Clone, Debug)]
pub struct Config {
    /// Width of the encompassing rectangle.
    pub width: i32,
    /// Height of the encompassing rectangle.
    pub height: i32,

    /// Minimum spacing between border and rectangles.
    pub border_padding: i32,
    /// Minimum spacing between rectangles.
    pub rectangle_padding: i32,

    /// Allow 90° rotation of the input rectangles.
    pub allow_rotation: bool,
}
