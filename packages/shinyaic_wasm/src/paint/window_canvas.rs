use crate::paint::border::Border;
use crate::paint::font::PaintFont;
use crate::render_tree::rectangle::Rectangle as RenderObjectRectangle;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn create_block(
    color: String,
    border: Border,
    rect: RenderObjectRectangle,
    canvas: &CanvasRenderingContext2d,
) {
    log(&format!(
        "hoge: {:?}, {:?}, {:?}, {:?}, {:?}",
        rect.x, rect.y, rect.width, rect.height, color
    ));
    canvas.set_fill_style(&color.into());
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
    log(&format!(
        "hoge: {:?}, {:?}, {:?}, {:?}, {:?}",
        rect.x, rect.y, rect.width, rect.height, content
    ));
    log(&format!("font: {:?}, {:?}", font.size, font.family_name));
    canvas.set_fill_style(&color.into());
    canvas.set_font(&format!("{:?}px {:?}", font.size, font.family_name));
    canvas.fill_text(&content, rect.x as f64, rect.y as f64);
}
