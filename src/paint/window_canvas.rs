use crate::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use iced_graphics::Primitive;
use iced_native::{Background, Color, Point, Rectangle, Size, Font, HorizontalAlignment, VerticalAlignment};

pub fn create_block(color: Color, rect: RenderObjectRectangle) -> Primitive {
    Primitive::Quad {
        bounds: Rectangle::new(
            Point::new(rect.x, rect.y),
            Size::new(rect.width, rect.height),
        ),
        background: Background::Color(Color::from_rgba8(
            color.r as u8,
            color.g as u8,
            color.b as u8,
            color.a,
        )),
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}

pub fn create_text(
    content: String,
    color: Color,
    rect: RenderObjectRectangle,
    font: Font
) -> Primitive {
    Primitive::Text {
        content,
        bounds: Rectangle::new(
            Point::new(rect.x, rect.y),
            Size::new(rect.width, rect.height),
        ),
        color: Color::from_rgba8(color.r as u8, color.g as u8, color.b as u8, color.a),
        size: 18.0,
        font,
        horizontal_alignment: HorizontalAlignment::Left,
        vertical_alignment: VerticalAlignment::Top,
    }
}
