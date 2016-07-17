#[derive(Copy, Clone)]
pub struct TexturePackerConfig {
    //
    // layout configuration
    //
    /// Max width of the packed image.
    pub max_width: u32,
    /// Max height of the packed image.
    pub max_height: u32,
    /// True to allow rotation (90°) of the input images.
    pub allow_rotation: bool,

    //
    // texture configuration
    //
    /// Size of the padding on the outer edge of the packed image in pixel.
    pub border_padding: u32,
    /// Size of the padding between frames in pixel.
    pub texture_padding: u32,
}
