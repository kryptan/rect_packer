#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    #[inline(always)]
    pub fn top(&self) -> u32 {
        self.y
    }

    #[inline(always)]
    pub fn bottom(&self) -> u32 {
        self.y + self.h
    }

    #[inline(always)]
    pub fn left(&self) -> u32 {
        self.x
    }

    #[inline(always)]
    pub fn right(&self) -> u32 {
        self.x + self.w
    }

    #[inline(always)]
    pub fn area(&self) -> u32 {
        self.w * self.h
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.contains_point(other.left(), other.top()) ||
        self.contains_point(other.left(), other.bottom() - 1) ||
        self.contains_point(other.right() - 1, other.bottom() - 1) ||
        self.contains_point(other.right() - 1, other.top()) ||
        other.contains_point(self.left(), self.top()) ||
        other.contains_point(self.left(), self.bottom() - 1) ||
        other.contains_point(self.right() - 1, self.bottom() - 1) ||
        other.contains_point(self.right() - 1, self.top())
    }

    pub fn contains(&self, other: &Rect) -> bool {
        self.left() <= other.left() &&
        self.right() >= other.right() &&
        self.top() <= other.top() &&
        self.bottom() >= other.bottom()
    }

    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        self.left() <= x && x < self.right() &&
        self.top() <= y && y < self.bottom()
    }
}
