use crate::paint::font::{FontContext, PaintFont};
use crate::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use iced_graphics::Primitive;
use iced_native::{
    Background, Color, HorizontalAlignment, Point, Rectangle, Size, VerticalAlignment,
};

pub fn create_block(color: Color, rect: RenderObjectRectangle) -> Primitive {
    Primitive::Quad {
        bounds: Rectangle::new(
            Point::new(rect.x, 45.0 + rect.y),
            Size::new(rect.width, rect.height),
        ),
        background: Background::Color(color),
        border_radius: 0.0,
        border_width: 1.0,
        border_color: Color::BLACK,
    }
}

pub fn create_text(
    content: String,
    color: Color,
    rect: RenderObjectRectangle,
    font: PaintFont,
) -> Primitive {
    Primitive::Text {
        content,
        bounds: Rectangle::new(
            Point::new(rect.x, 45.0 + rect.y),
            Size::new(rect.width, rect.height),
        ),
        color: Color::from_rgba8(color.r as u8, color.g as u8, color.b as u8, color.a),
        size: font.size,
        font: font.to_iced_font(&mut FontContext::new()),
        horizontal_alignment: HorizontalAlignment::Left,
        vertical_alignment: VerticalAlignment::Top,
    }
}
