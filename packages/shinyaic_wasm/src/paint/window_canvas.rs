use crate::paint::border::Border;
use crate::paint::font::PaintFont;
use crate::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use web_sys::CanvasRenderingContext2d;

pub fn create_block(
    color: String,
    border: Border,
    rect: RenderObjectRectangle,
    canvas: &CanvasRenderingContext2d,
) {
    let mut style = canvas.fill_style();
    // style = color;
    canvas.fill_rect(
        rect.x as f64,
        rect.y as f64,
        rect.width as f64,
        rect.height as f64,
    );
}

pub fn create_text(
    content: String,
    color: String,
    rect: RenderObjectRectangle,
    font: PaintFont,
    canvas: &CanvasRenderingContext2d,
) {
    canvas.fill_text(&content, rect.x as f64, rect.y as f64);
}
