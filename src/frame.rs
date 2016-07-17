use rect::Rect;

#[derive(Clone, Debug)]
pub struct Frame {
    pub key: String,
    pub frame: Rect,
    pub rotated: bool,
}
