#[derive(Clone, Debug, PartialEq)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    pub fn new(width: f32, height: f32) -> WindowSize {
        WindowSize {
            width: width,
            height: height,
        }
    }
}
