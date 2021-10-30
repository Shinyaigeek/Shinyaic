use crate::paint::border::Border;
use crate::paint::font::{FontContext, PaintFont};
use shinyaic_core::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use iced_graphics::Primitive;
use iced_native::{
    Background, Color, HorizontalAlignment, Point, Rectangle, Size, VerticalAlignment,
};

struct InteractiveStyle {
    shadow_offset: Vec<f32>,
    background: Option<Background>,
    border_radius: u16,
    border_width: u16,
    border_color: Color,
    text_color: Color,
}

trait InteractiveBlock {
    fn hovered(&self) -> InteractiveStyle {
        InteractiveStyle {
            shadow_offset: vec![0.0, 0.0],
            background: None,
            border_radius: 0,
            border_width: 1,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            text_color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

impl InteractiveBlock for Primitive {
    fn hovered(&self) -> InteractiveStyle {
        InteractiveStyle {
            shadow_offset: vec![0.0, 0.0],
            background: None,
            border_radius: 0,
            border_width: 1,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            text_color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

pub fn create_block(color: Color, border: Border, rect: RenderObjectRectangle) -> Primitive {
    Primitive::Quad {
        bounds: Rectangle::new(
            Point::new(rect.x, 45.0 + rect.y),
            Size::new(rect.width, rect.height),
        ),
        background: Background::Color(color),
        border_radius: border.radius,
        border_width: border.width,
        border_color: border.color,
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
