use std;
use std::cmp::max;

use {Config, Rect};

#[derive(Clone)]
struct Skyline {
    pub x: u32,
    pub y: u32,
    pub w: u32,
}

impl Skyline {
    #[inline(always)]
    pub fn left(&self) -> u32 {
        self.x
    }

    #[inline(always)]
    pub fn right(&self) -> u32 {
        self.x + self.w
    }
}

#[derive(Clone)]
pub struct Packer {
    config: Config,
    border: Rect,

    // the skylines are sorted by their `x` position
    skylines: Vec<Skyline>,
}

impl Packer {
    pub fn new(config: Config) -> Packer {
        let width = config.width + config.rectangle_padding - 2*config.border_padding;
        let height = config.height + config.rectangle_padding - 2*config.border_padding;

        let skylines = vec![Skyline {
            x: 0,
            y: 0,
            w: width,
        }];

        Packer {
            config: config,
            border: Rect::new(0, 0, width, height),
            skylines: skylines,
        }
    }

    pub fn pack(&mut self, (width, height): (u32, u32)) -> Option<Rect> {
        let padded_width = width + self.config.rectangle_padding;
        let padded_height = height + self.config.rectangle_padding;

        if let Some((i, mut rect)) = self.find_skyline(padded_width, padded_height) {
            self.split(i, &rect);
            self.merge();

            rect.w -= self.config.rectangle_padding;
            rect.h -= self.config.rectangle_padding;
            rect.x += self.config.border_padding;
            rect.y += self.config.border_padding;

            Some(rect)
        } else {
            None
        }
    }

    pub fn can_pack(&self, (width, height): (u32, u32)) -> bool {
        self.find_skyline(width + self.config.rectangle_padding, height + self.config.rectangle_padding).is_some()
    }

    // return `rect` if rectangle (w, h) can fit the skyline started at `i`
    fn can_put(&self, mut i: usize, w: u32, h: u32) -> Option<Rect> {
        let mut rect = Rect::new(self.skylines[i].x, 0, w, h);
        let mut width_left = rect.w;
        loop {
            rect.y = max(rect.y, self.skylines[i].y);
            // the source rect is too large
            if !self.border.contains(&rect) {
                return None;
            }
            if self.skylines[i].w >= width_left {
                return Some(rect);
            }
            width_left -= self.skylines[i].w;
            i += 1;
            assert!(i < self.skylines.len());
        }
    }

    fn find_skyline(&self, w: u32, h: u32) -> Option<(usize, Rect)> {
        let mut bottom = std::u32::MAX;
        let mut width = std::u32::MAX;
        let mut index = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        // keep the `bottom` and `width` as small as possible
        for i in 0..self.skylines.len() {
            if let Some(r) = self.can_put(i, w, h) {
                if r.bottom() < bottom ||
                    (r.bottom() == bottom && self.skylines[i].w < width) {
                    bottom = r.bottom();
                    width = self.skylines[i].w;
                    index = Some(i);
                    rect = r;
                }
            }

            if self.config.allow_rotation {
                if let Some(r) = self.can_put(i, h, w) {
                    if r.bottom() < bottom ||
                        (r.bottom() == bottom && self.skylines[i].w < width) {
                        bottom = r.bottom();
                        width = self.skylines[i].w;
                        index = Some(i);
                        rect = r;
                    }
                }
            }
        }

        if let Some(index) = index {
            Some((index, rect))
        } else {
            None
        }
    }

    fn split(&mut self, i: usize, rect: &Rect) {
        let skyline = Skyline {
            x: rect.left(),
            y: rect.bottom(),
            w: rect.w,
        };

        assert!(skyline.right() <= self.border.right());
        assert!(skyline.y <= self.border.bottom());

        self.skylines.insert(i, skyline);

        while i + 1 < self.skylines.len() {
            assert!(self.skylines[i].left() <= self.skylines[i + 1].left());

            if self.skylines[i + 1].left() >= self.skylines[i].right() {
                break;
            }

            let shrink = self.skylines[i].right() - self.skylines[i + 1].left();
            if self.skylines[i + 1].w <= shrink {
                self.skylines.remove(i + 1);
            } else {
                self.skylines[i + 1].x += shrink;
                self.skylines[i + 1].w -= shrink;
                break;
            }
        }
    }

    fn merge(&mut self) {
        let mut i = 1;
        while i < self.skylines.len() {
            if self.skylines[i - 1].y == self.skylines[i].y {
                self.skylines[i - 1].w += self.skylines[i].w;
                self.skylines.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
