/// Rectangle with integer coordinates.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    #[inline(always)]
    pub fn top(&self) -> i32 {
        self.y
    }

    #[inline(always)]
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    #[inline(always)]
    pub fn left(&self) -> i32 {
        self.x
    }

    #[inline(always)]
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    #[inline(always)]
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    /// Check if intersection of two rectangles is non empty.
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

    /// Check if `other` rectangle is completely inside `self`.
    pub fn contains(&self, other: &Rect) -> bool {
        self.left() <= other.left() &&
        self.right() >= other.right() &&
        self.top() <= other.top() &&
        self.bottom() >= other.bottom()
    }

    /// Check if given pixel is inside this rectangle.
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.left() <= x && x < self.right() &&
        self.top() <= y && y < self.bottom()
    }
}
