use image;
use image::{
    GenericImage,
};

use texture::{
    Texture,
};

impl<P: image::Pixel, I: GenericImage<Pixel=P>> Texture for I {
    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn height(&self) -> u32 {
        self.dimensions().1
    }
}
