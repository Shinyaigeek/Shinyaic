use iced_native::Color;

pub struct Border {
    width: f32,
    radius: f32,
    color: Color,
}

impl Border {
    pub fn new(width: Option<f32>, radius: Option<f32>, color: Option<Color>) -> Self {
        let width = width.unwrap_or(0.0);
        let radius = radius.unwrap_or(0.0);
        let color = color.unwrap_or(Color::BLACK);
        Self {
            width,
            radius,
            color,
        }
    }
}
