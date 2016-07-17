use std::cmp::max;

use {Config, Rect};

mod packer;

#[derive(Clone)]
pub struct Packer {
    config: Config,

    packer : packer::Packer,
}

impl Packer {
    pub fn new(config: Config) -> Packer {
        let width = max(0, config.width + config.rectangle_padding - 2*config.border_padding);
        let height = max(0, config.height + config.rectangle_padding - 2*config.border_padding);

        Packer {
            config: config,
            packer: packer::Packer::new(width, height, config.allow_rotation),
        }
    }

    pub fn pack(&mut self, width : i32, height : i32) -> Option<Rect> {
        if width <= 0 || height <= 0 {
            return None
        }

        if let Some(mut rect) = self.packer.pack(width + self.config.rectangle_padding, height + self.config.rectangle_padding) {
            rect.w -= self.config.rectangle_padding;
            rect.h -= self.config.rectangle_padding;
            rect.x += self.config.border_padding;
            rect.y += self.config.border_padding;

            Some(rect)
        } else {
            None
        }
    }

    pub fn can_pack(&self, width : i32, height : i32) -> bool {
        self.packer.can_pack(width + self.config.rectangle_padding, height + self.config.rectangle_padding)
    }
}
