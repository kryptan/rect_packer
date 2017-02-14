use std;
use std::cmp::max;

use Rect;

#[derive(Clone)]
struct Skyline {
    pub left: i32,
    pub y: i32,
    pub width: i32,
}

impl Skyline {
    #[inline(always)]
    pub fn right(&self) -> i32 {
        self.left + self.width
    }
}

/// Similar to `Packer` but does not add any padding between rectangles.
#[derive(Clone)]
pub struct DensePacker {
    width: i32,
    height: i32,

    // the skylines are sorted by their `x` position
    skylines: Vec<Skyline>,
}

impl DensePacker {
    /// Create new empty `DensePacker` with the provided parameters.
    pub fn new(width : i32, height : i32) -> DensePacker {
        let width = max(0, width);
        let height = max(0, height);

        let skylines = vec![Skyline {
            left: 0,
            y: 0,
            width: width,
        }];

        DensePacker {
            width: width,
            height: height,
            skylines: skylines,
        }
    }

    /// Get size that this packer was created with.
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
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

        if let Some((i, rect)) = self.find_skyline(width, height, allow_rotation) {
            self.split(i, &rect);
            self.merge();

            Some(rect)
        } else {
            None
        }
    }

    /// Check if rectangle with the specified size can be added.
    pub fn can_pack(&self, width : i32, height : i32, allow_rotation : bool) -> bool {
        self.find_skyline(width, height, allow_rotation).is_some()
    }

    // return `rect` if rectangle (w, h) can fit the skyline started at `i`
    fn can_put(&self, mut i: usize, w: i32, h: i32) -> Option<Rect> {
        let mut rect = Rect::new(self.skylines[i].left, 0, w, h);
        let mut width_left = rect.width;
        loop {
            rect.y = max(rect.y, self.skylines[i].y);
            // the source rect is too large
            if !Rect::new(0, 0, self.width, self.height).contains(&rect) {
                return None;
            }
            if self.skylines[i].width >= width_left {
                return Some(rect);
            }
            width_left -= self.skylines[i].width;
            i += 1;
            assert!(i < self.skylines.len());
        }
    }

    fn find_skyline(&self, w: i32, h: i32, allow_rotation : bool) -> Option<(usize, Rect)> {
        let mut bottom = std::i32::MAX;
        let mut width = std::i32::MAX;
        let mut index = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        // keep the `bottom` and `width` as small as possible
        for i in 0..self.skylines.len() {
            if let Some(r) = self.can_put(i, w, h) {
                if r.bottom() < bottom ||
                    (r.bottom() == bottom && self.skylines[i].width < width) {
                    bottom = r.bottom();
                    width = self.skylines[i].width;
                    index = Some(i);
                    rect = r;
                }
            }

            if allow_rotation {
                if let Some(r) = self.can_put(i, h, w) {
                    if r.bottom() < bottom ||
                        (r.bottom() == bottom && self.skylines[i].width < width) {
                        bottom = r.bottom();
                        width = self.skylines[i].width;
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
            left: rect.left(),
            y: rect.bottom(),
            width: rect.width,
        };

        assert!(skyline.right() <= self.width);
        assert!(skyline.y <= self.height);

        self.skylines.insert(i, skyline);

        while i + 1 < self.skylines.len() {
            assert!(self.skylines[i].left <= self.skylines[i + 1].left);

            if self.skylines[i + 1].left >= self.skylines[i].right() {
                break;
            }

            let shrink = self.skylines[i].right() - self.skylines[i + 1].left;
            if self.skylines[i + 1].width <= shrink {
                self.skylines.remove(i + 1);
            } else {
                self.skylines[i + 1].left += shrink;
                self.skylines[i + 1].width -= shrink;
                break;
            }
        }
    }

    fn merge(&mut self) {
        let mut i = 1;
        while i < self.skylines.len() {
            if self.skylines[i - 1].y == self.skylines[i].y {
                self.skylines[i - 1].width += self.skylines[i].width;
                self.skylines.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
