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

    let text_rect = canvas.measure_text(&content).unwrap();
    let mut text_width = text_rect.width();
    let height = text_rect.actual_bounding_box_descent() + text_rect.actual_bounding_box_ascent();
    let mut diff_y = 0.0;

    let mut content = content.clone();

    if text_width < (rect.width as f64) {
        canvas.fill_text(&content, rect.x as f64, rect.y as f64);
    } else {
        let mut text = content.chars().peekable();
        let mut current_line_text = String::from("");
        loop {
            log(&format!(
                "l: {:?}, {:?}, {:?}",
                text.peek().is_none(),
                current_line_text,
                text.peek().unwrap_or(&'a')
            ));

            if text.peek().is_none() {
                canvas.fill_text(&current_line_text, rect.x as f64, (rect.y as f64 + diff_y));
                break;
            }

            let c = text.peek().unwrap();

            if canvas
                .measure_text(&format!("{:?}{:?}", current_line_text, c))
                .unwrap()
                .width()
                > rect.width as f64
            {
                canvas.fill_text(&current_line_text, rect.x as f64, (rect.y as f64 + diff_y));
                log(&format!("log: {:?}, {:?}", current_line_text, diff_y));
                diff_y += height;
                current_line_text = String::from("");
                continue;
            }

            current_line_text.push(*c);
            text.next();
        }
    }
}
