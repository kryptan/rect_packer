#[cfg(test)]
extern crate rand;

#[cfg(test)]
extern crate image;

pub use rect::Rect;
pub use packer::SkylinePacker;

mod rect;
mod packer;

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct Frame {
    pub frame: Rect,
    pub rotated: bool,
}

#[derive(Copy, Clone)]
pub struct Config {
    //
    // layout configuration
    //
    /// Max width of the packed image.
    pub width: u32,
    /// Max height of the packed image.
    pub height: u32,
    /// Allow rotation (90°) of the input images.
    pub allow_rotation: bool,

    //
    // texture configuration
    //
    /// Size of the padding on the outer edge of the packed image in pixel.
    pub border_padding: u32,
    /// Size of the padding between frames in pixel.
    pub texture_padding: u32,
}
