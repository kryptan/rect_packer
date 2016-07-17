use texture::{
    Texture,
};
use frame::Frame;

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer {
    fn pack(&mut self, key: String, texture: &Texture) -> Option<Frame>;
    fn can_pack(&self, texture: &Texture) -> bool;
}
