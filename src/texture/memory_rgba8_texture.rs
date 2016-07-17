use texture::{
    Texture,
};

#[derive(Copy, Clone)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct MemoryRGBA8Texture {
    width: u32,
    height: u32,
}

impl MemoryRGBA8Texture {
    pub fn from_memory(buf: &[u8], w: u32, h: u32) -> MemoryRGBA8Texture {
        let mut pixels = Vec::new();

        for pixel in buf.chunks(4) {
            pixels.push(RGBA8 {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }

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
