use std::collections::HashMap;

use {
    TexturePackerConfig,
};

use rect::Rect;

use texture::{
    Texture,
    SubTexture,
};

use frame::Frame;
use packer::{
    Packer,
    SkylinePacker,
};

pub struct TexturePacker<P> {
    textures: HashMap<String, SubTexture>,
    frames: HashMap<String, Frame>,
    packer: P,
    config: TexturePackerConfig,
}

impl <'a, Pix> TexturePacker<SkylinePacker<Pix>> {
    pub fn new_skyline(config: TexturePackerConfig) -> TexturePacker<SkylinePacker<Pix>> {
        TexturePacker {
            textures: HashMap::new(),
            frames: HashMap::new(),
            packer: SkylinePacker::<Pix>::new(config),
            config: config,
        }
    }
}

impl<'a, P: Packer> TexturePacker<P> {
    pub fn pack_ref<T: Clone + Texture>(&mut self, key: String, texture: &'a T) {
        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::from_ref(source);
        if let Some(mut frame) = self.packer.pack(key.clone(), &texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            frame.trimmed = self.config.trim;
            frame.source = source;
            frame.source.w = w;
            frame.source.h = h;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, texture);
    }

    pub fn pack_own<T: Clone + Texture>(&mut self, key: String, texture: T) {
        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(&texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::new(source);
        if let Some(mut frame) = self.packer.pack(key.clone(), &texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            frame.trimmed = self.config.trim;
            frame.source = source;
            frame.source.w = w;
            frame.source.h = h;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, texture);
    }

    pub fn get_frames(&self) -> &HashMap<String, Frame> {
        &self.frames
    }

    pub fn get_frame(&self, key: &str) -> Option<&Frame> {
        if let Some(frame) = self.frames.get(key) {
            Some(frame)
        } else {
            None
        }
    }
}

impl<'a, P> Texture for  TexturePacker<P>
where P: Packer {
    fn width(&self) -> u32 {
        let mut right = None;

        for (_, frame) in self.frames.iter() {
            if let Some(r) = right {
                if frame.frame.right() > r {
                    right = Some(frame.frame.right());
                }
            } else {
                right = Some(frame.frame.right());
            }
        }

        if let Some(right) = right {
            right + 1 + self.config.border_padding
        } else {
            0
        }
    }

    fn height(&self) -> u32 {
        let mut bottom = None;

        for (_, frame) in self.frames.iter() {
            if let Some(b) = bottom {
                if frame.frame.bottom() > b {
                    bottom = Some(frame.frame.bottom());
                }
            } else {
                bottom = Some(frame.frame.bottom());
            }
        }

        if let Some(bottom) = bottom {
            bottom + 1 + self.config.border_padding
        } else {
            0
        }
    }
}

fn trim_texture<T: Texture>(texture: &T) -> Rect {
    Rect::new_with_points(0, 0, texture.width() - 1, texture.height() - 1)
}
