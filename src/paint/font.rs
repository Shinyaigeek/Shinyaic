use font_kit::canvas::RasterizationOptions;
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::{Properties as FontProperties, Style as FontStyle};
use font_kit::source::SystemSource;
use iced_native::Font as IcedFont;
use pathfinder_geometry::transform2d::Transform2F;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct FontContext {
    font_data_caches: HashMap<FontCacheKey, &'static [u8]>,
}

impl FontContext {
    pub fn new() -> FontContext {
        FontContext {
            font_data_caches: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PaintFont {
    pub font: Font,
    pub size: f32,
    pub ascent: f32,
    pub descent: f32,
    pub family_name: String,
    cache_key: FontCacheKey,
}

impl PartialEq for PaintFont {
    fn eq(&self, other: &Self) -> bool {
        self.family_name == other.family_name
    }
}

#[derive(Clone, Debug)]
pub struct FontCacheKey {
    size: f32,
    properties: FontProperties,
    family_name: String,
}

impl Hash for FontCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.size as i32).hash(state);
        match self.properties.style {
            FontStyle::Normal => "normal".hash(state),
            FontStyle::Italic => "italic".hash(state),
            FontStyle::Oblique => "oblique".hash(state),
        }
        (self.properties.stretch.0 as i32).hash(state);
        (self.properties.weight.0 as i32).hash(state);
        self.family_name.hash(state);
    }
}

impl PartialEq for FontCacheKey {
    fn eq(&self, other: &Self) -> bool {
        (self.size as i32) == (other.size as i32)
            && (self.properties.stretch.0 as i32) == (other.properties.stretch.0 as i32)
            && (self.properties.weight.0 as i32) == (other.properties.weight.0 as i32)
            && matches!(
                (self.properties.style, other.properties.style),
                (FontStyle::Normal, FontStyle::Normal)
                    | (FontStyle::Italic, FontStyle::Italic)
                    | (FontStyle::Oblique, FontStyle::Oblique)
            )
            && self.family_name == other.family_name
    }
}

impl Eq for FontCacheKey {}

impl FontCacheKey {
    pub fn new(size: f32, properties: FontProperties, family_name: String) -> FontCacheKey {
        FontCacheKey {
            size,
            properties,
            family_name,
        }
    }
}

impl PaintFont {
    pub fn new(family: Option<String>, size: Option<f32>) -> Self {
        let size = size.unwrap_or(18.0);
        let font_families = family.unwrap_or("default".to_string());

        let font = SystemSource::new()
            .select_best_match(&[FamilyName::Serif], &FontProperties::new())
            .unwrap()
            .load()
            .unwrap();

        let ctfont = font.native_font().clone_with_font_size(size as f64);

        let ascent = ctfont.ascent() as f64;
        let descent = ctfont.descent() as f64;

        let scale = px_to_pt(ctfont.pt_size() as f64) / (ascent + descent);

        Self {
            font,
            ascent: pt_to_px(ascent * scale) as f32,
            descent: pt_to_px(descent * scale) as f32,
            size,
            family_name: font_families.clone(),
            cache_key: FontCacheKey::new(size, FontProperties::new(), font_families),
        }
    }

    pub fn get_static_font_data(&self, font_context: &mut FontContext) -> &'static [u8] {
        if let Some(data) = font_context.font_data_caches.get(&self.cache_key) {
            return data;
        }
        let font_data = &*self.font.copy_font_data().unwrap();
        let boxed_slice = font_data.clone().into_boxed_slice();
        let leaked_slice = Box::leak(boxed_slice);
        font_context
            .font_data_caches
            .insert(self.cache_key.clone(), leaked_slice);
        leaked_slice
    }

    pub fn get_static_hashed_family_name(&self) -> &'static str {
        let mut hasher = DefaultHasher::new();
        self.cache_key.hash(&mut hasher);
        Box::leak(hasher.finish().to_string().into_boxed_str())
    }

    pub fn to_iced_font(&self, font_context: &mut FontContext) -> IcedFont {
        IcedFont::External {
            name: self.get_static_hashed_family_name(),
            bytes: self.get_static_font_data(font_context),
        }
    }

    pub fn get_font_rendered_size(&self, width: f32, text: String) -> PaintFontRenderedRect {
        let ctfont = self
            .font
            .native_font()
            .clone_with_font_size(self.size as f64);
        let font = ctfont.bounding_box();

        // TODO: これでいいの感
        let id = self.font.glyph_for_char('a').unwrap();

        let bounding = self
            .font
            .raster_bounds(
                id,
                self.size,
                Transform2F::default(),
                HintingOptions::None,
                RasterizationOptions::Bilevel,
            )
            .unwrap();

        let (width, height): (f32, f32) = {
            if (text.len() as f32) * (bounding.width() as f32) <= width {
                (bounding.width() as f32, font.size.height as f32)
            } else {
                let mut height = 0.0;
                let mut text_cnt = text.len() as isize;
                loop {
                    text_cnt -= (width / bounding.width() as f32) as isize + 1;
                    height += font.size.height as f32;

                    if text_cnt <= 0 {
                        break;
                    }
                }

                (width, height)
            }
        };

        println!("font: {:#?}", font);

        PaintFontRenderedRect {
            x: font.origin.x,
            y: font.origin.y,
            width: width as f64,
            height: height as f64,
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
