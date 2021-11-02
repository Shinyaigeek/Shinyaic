use crate::wasm_bindgen::JsCast;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone, Debug)]
pub struct PaintFont {
    pub size: f32,
    pub ascent: f32,
    pub descent: f32,
    pub family_name: String,
    canvas_context: CanvasRenderingContext2d,
}

impl PartialEq for PaintFont {
    fn eq(&self, other: &Self) -> bool {
        self.family_name == other.family_name
    }
}

impl PaintFont {
    pub fn new(family: Option<String>, size: Option<f32>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Self {
            ascent: 18.0,
            descent: 18.0,
            size: size.unwrap_or(18.0),
            family_name: family.unwrap_or("sans-serif".to_string()),
            canvas_context: context,
        }
    }

    pub fn get_font_rendered_size(&self, width: f32, text: String) -> PaintFontRenderedRect {
        self.canvas_context
            .set_font(&format!("{:?}px {:?}", self.size, self.family_name));
        let rect = self.canvas_context.measure_text(&text).unwrap();
        let mut rendered_width = rect.width().clone();
        let mut rendered_height = 0.0;
        // TODO version update
        let rect_height = rect.actual_bounding_box_descent() + rect.actual_bounding_box_ascent();
        if rendered_width < (width as f64) {
            return PaintFontRenderedRect {
                x: 0.0,
                y: 0.0,
                width: rect.width() as f64,
                height: rect_height as f64,
            };
        } else {
            loop {
                rendered_width -= width as f64;
                rendered_height += rect_height;

                if rendered_width < (width as f64) {
                    return PaintFontRenderedRect {
                        x: 0.0,
                        y: 0.0,
                        width: width as f64,
                        height: (rendered_height + rect_height + rect_height) as f64,
                    };
                }
            }
        }
    }
}

pub struct PaintFontRenderedRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

fn px_to_pt(px: f64) -> f64 {
    px / 96. * 72.
}

fn pt_to_px(pt: f64) -> f64 {
    pt / 72. * 96.
}
