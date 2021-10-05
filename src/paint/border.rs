use iced_native::Color;

pub struct Border {
    width: f32,
    radius: f32,
    style: BorderStyle,
    color: Color,
}

impl Border {
    pub fn new(
        width: Option<f32>,
        radius: Option<f32>,
        color: Option<Color>,
        style: Option<&str>,
    ) -> Self {
        let width = width.unwrap_or(0.0);
        let radius = radius.unwrap_or(0.0);
        let color = color.unwrap_or(Color::BLACK);
        let style = BorderStyle::new(style.unwrap_or("none"));
        Self {
            width,
            radius,
            color,
            style,
        }
    }

    pub fn is_color(style: &str) -> bool {
        // todo
        !(Self::is_style(style) || Self::is_width(style))
    }

    pub fn apply_color(&mut self, color: &str) {
        // TODO common color processing
        let color = match color {
            "black" => Color::BLACK,
            _ => Color::BLACK,
        };

        self.color = color;
    }

    fn is_width(width: &str) -> bool {
        match width {
            "thin" | "thick" | "medium" => true,
            _ => {
                if width.parse::<f32>().is_ok() {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn apply_width(&mut self, width: &str) {
        let width = match width {
            "thin" => 0.5,
            "medium" => 1.0,
            "thick" => 3.0,
            _ => width.parse::<f32>().unwrap(),
        };

        self.width = width;
    }

    pub fn is_style(style: &str) -> bool {
        BorderStyle::is_style(style)
    }

    pub fn apply_style(&mut self, style: &str) {
        let style = BorderStyle::new(style);
        self.style = style;
    }

    pub fn apply_shorthand(&mut self, shorthand: &str) {
        let styles = shorthand.split(" ");

        for style in styles {
            if Self::is_style(style) {
                self.apply_style(style);
            } else if Self::is_width(style) {
                self.apply_width(style)
            } else if Self::is_color(style) {
                self.apply_color(style)
            } else {
                panic!("TODO")
            }
        }
    }
}

enum BorderStyle {
    None,
    Groove,
    Dotted,
    Ridge,
    Dashed,
    Inset,
    Solid,
    Outset,
    Double,
}

impl BorderStyle {
    pub fn new(style: &str) -> Self {
        match style {
            "none" => Self::None,
            "groove" => Self::Groove,
            "dotted" => Self::Dotted,
            "ridge" => Self::Ridge,
            "dashed" => Self::Dashed,
            "inset" => Self::Inset,
            "solid" => Self::Solid,
            "outset" => Self::Outset,
            "double" => Self::Double,
            _ => Self::None,
        }
    }

    pub fn is_style(style: &str) -> bool {
        match style {
            "none" => true,
            "groove" => true,
            "dotted" => true,
            "ridge" => true,
            "dashed" => true,
            "inset" => true,
            "solid" => true,
            "outset" => true,
            "double" => true,
            _ => false,
        }
    }
}
