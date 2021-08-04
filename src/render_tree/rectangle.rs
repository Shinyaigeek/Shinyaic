// TODO width, height 0 が auto の意味合いを持ってしまう
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}
