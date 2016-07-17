use image;
use image::{
    GenericImage,
};

use texture::{
    Texture,
};

impl<P: image::Pixel, I: GenericImage<Pixel=P>> Texture for I {
    type Pixel = I::Pixel;

    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn height(&self) -> u32 {
        self.dimensions().1
    }

    fn get(&self, x: u32, y: u32) -> Option<I::Pixel> {
        if self.in_bounds(x, y) {
            Some(self.get_pixel(x, y))
        } else {
            None
        }
    }

    fn set(&mut self, x: u32, y: u32, val: I::Pixel) {
        self.put_pixel(x, y, val);
    }
}
