// TODO width, height 0 が auto の意味合いを持ってしまう
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}
