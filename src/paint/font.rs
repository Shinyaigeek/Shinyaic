use iced_native::{Font, HorizontalAlignment, VerticalAlignment};

pub struct PaintFont {
    pub font: Font,
    pub size: f32,
    pub text_align: HorizontalAlignment,
    pub vertical_align: VerticalAlignment,
}

impl PaintFont {
    pub fn new(
        font: Option<Font>,
        size: Option<f32>,
        text_align: Option<HorizontalAlignment>,
        vertical_align: Option<VerticalAlignment>,
    ) -> Self {
        let font = font.unwrap_or(Font::default());
        let size = size.unwrap_or(18.0);
        let text_align = text_align.unwrap_or(HorizontalAlignment::Left);
        let vertical_align = vertical_align.unwrap_or(VerticalAlignment::Top);
        Self {
            font,
            size,
            text_align,
            vertical_align,
        }
    }
}
