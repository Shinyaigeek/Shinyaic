use core_text::font::CTFont;
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::{
    Properties as FontProperties, Stretch as FontStretch, Style as FontStyle, Weight as FontWeight,
};
use font_kit::source::SystemSource;
use iced_native::Font as IcedFont;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct FontContext {
    font_caches: HashMap<FontCacheKey, PaintFont>,
    font_data_caches: HashMap<FontCacheKey, &'static [u8]>,
}

impl FontContext {
    pub fn new() -> FontContext {
        FontContext {
            font_caches: HashMap::new(),
            font_data_caches: HashMap::new(),
        }
    }

    pub fn get_or_create_by(&mut self, cache_key: &FontCacheKey) -> PaintFont {
        let font = self.font_caches.get(&cache_key);
        if let Some(font) = font {
            return font.clone();
        }
        let font = PaintFont::new(None, None);
        self.font_caches.insert(cache_key.clone(), font.clone());
        font
    }
}

#[derive(Clone, Debug)]
pub struct PaintFont {
    pub font: Font,
    pub size: f32,
    pub ascent: f32,
    pub descent: f32,
    pub family_name: String,
    ctfont: CTFont,
    units_per_em: f32,
    cache_key: FontCacheKey,
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

    pub fn new_from_style() -> FontCacheKey {
        FontCacheKey {
            size: 18.0,
            properties: create_font_properties(),
            // TODO: Fix to find appropriate family name
            family_name: "".to_string(),
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
            units_per_em: ctfont.units_per_em() as f32,
            ctfont,
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
}

fn px_to_pt(px: f64) -> f64 {
    px / 96. * 72.
}

fn pt_to_px(pt: f64) -> f64 {
    pt / 72. * 96.
}

pub fn create_font_properties() -> FontProperties {
    let style = FontStyle::Normal;
    let weight = FontWeight(100.0);

    FontProperties {
        style,
        weight,
        stretch: FontStretch::NORMAL,
    }
}
