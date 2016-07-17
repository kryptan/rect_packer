use frame::Frame;

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer {
    fn pack(&mut self, key: String, size: (u32, u32)) -> Option<Frame>;
    fn can_pack(&self, size: (u32, u32)) -> bool;
}
