use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct PaintFont {
    pub size: f32,
    pub ascent: f32,
    pub descent: f32,
    pub family_name: String,
}

impl PartialEq for PaintFont {
    fn eq(&self, other: &Self) -> bool {
        self.family_name == other.family_name
    }
}

impl PaintFont {
    pub fn new(family: Option<String>, size: Option<f32>) -> Self {
        Self {
            ascent: 18.0,
            descent: 18.0,
            size: 18.0,
            family_name: "asdf".to_string(),
        }
    }

    pub fn get_font_rendered_size(&self, width: f32, text: String) -> PaintFontRenderedRect {
        PaintFontRenderedRect {
            x: 18.0,
            y: 18.0,
            width: 18.0 as f64,
            height: 18.0 as f64,
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
