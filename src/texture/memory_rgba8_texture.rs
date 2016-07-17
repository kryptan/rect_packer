use texture::Texture;

pub struct MemoryRGBA8Texture {
    width: u32,
    height: u32,
}

impl MemoryRGBA8Texture {
    pub fn new(w: u32, h: u32) -> MemoryRGBA8Texture {
        MemoryRGBA8Texture {
            width: w,
            height: h,
        }
    }
}

impl Texture for MemoryRGBA8Texture {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
