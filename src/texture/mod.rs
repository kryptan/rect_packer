pub use self::memory_rgba8_texture::MemoryRGBA8Texture;
pub use self::sub_texture::SubTexture;

use std::ops::{Deref};

pub mod sub_texture;
pub mod memory_rgba8_texture;
pub mod image_texture;

pub trait Texture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

}

impl Texture for Box<Texture + 'static> {
    fn width(&self) -> u32 {
        self.deref().width()
    }

    fn height(&self) -> u32 {
        self.deref().height()
    }
}
