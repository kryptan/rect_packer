use std::cmp::max;
use {Config, Rect};

mod packer;

pub use self::packer::DensePacker;

/// `Packer` is the main structure in this crate. It holds packing context.
#[derive(Clone)]
pub struct Packer {
    config: Config,
    packer : DensePacker,
}

impl Packer {
    /// Create new empty `Packer` with the provided parameters.
    pub fn new(config: Config) -> Packer {
        let width = max(0, config.width + config.rectangle_padding - 2*config.border_padding);
        let height = max(0, config.height + config.rectangle_padding - 2*config.border_padding);

        Packer {
            config: config,
            packer: DensePacker::new(width, height),
        }
    }

    /// Get config that this packer was created with.
    pub fn config(&self) -> Config {
        self.config
    }

    /// Pack new rectangle. Returns position of the newly added rectangle. If there is not enough space returns `None`.
    /// If it returns `None` you can still try to add smaller rectangles.
    ///
    /// `allow_rotation` - allow 90° rotation of the input rectangle. You can detect whether rectangle was rotated by comparing
    /// returned `width` and `height` with the supplied ones.
    pub fn pack(&mut self, width : i32, height : i32, allow_rotation : bool) -> Option<Rect> {
        if width <= 0 || height <= 0 {
            return None
        }

        if let Some(mut rect) = self.packer.pack(width + self.config.rectangle_padding, height + self.config.rectangle_padding, allow_rotation) {
            rect.width -= self.config.rectangle_padding;
            rect.height -= self.config.rectangle_padding;
            rect.x += self.config.border_padding;
            rect.y += self.config.border_padding;

            Some(rect)
        } else {
            None
        }
    }

    /// Check if rectangle with the specified size can be added.
    pub fn can_pack(&self, width : i32, height : i32, allow_rotation : bool) -> bool {
        self.packer.can_pack(width + self.config.rectangle_padding, height + self.config.rectangle_padding, allow_rotation)
    }
}
