use rect::Rect;

use super::Texture;

pub struct SubTexture {
    source: Rect,
}

impl SubTexture {
    pub fn new(source: Rect) -> SubTexture {
        SubTexture {
            source: source,
        }
    }

    pub fn from_ref(source: Rect) -> SubTexture {
        SubTexture {
            source: source,
        }
    }

}

impl Texture for SubTexture {
    fn width(&self) -> u32 {
        self.source.w
    }

    fn height(&self) -> u32 {
        self.source.h
    }
}

