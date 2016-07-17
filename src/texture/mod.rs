pub use self::memory_rgba8_texture::MemoryRGBA8Texture;
pub use self::sub_texture::SubTexture;

pub mod sub_texture;
pub mod memory_rgba8_texture;

pub trait Texture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

