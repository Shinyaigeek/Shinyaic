use iced::Color;
use shinyaic_core::css::cssom::cssom::StylingRule;

pub fn handle_background(style: &StylingRule) -> Option<Color> {
    let background_color_value = {
        if style.declarations.get(&"background".to_string()).is_some() {
            style.declarations.get(&"background".to_string())
        } else if style
            .declarations
            .get(&"background-color".to_string())
            .is_some()
        {
            style.declarations.get(&"background-color".to_string())
        } else {
            None
        }
    };

    if background_color_value.is_some() {
        let mut raw_background_color = background_color_value.unwrap().clone();
        let colors = if raw_background_color.starts_with("#") {
            // TODO other module
            let mut colors = vec![];

            let color = raw_background_color.strip_prefix("#").unwrap().to_string();
            if color.len() == 3 {
                let c = &color[0..1];
                let mut c = c.to_string();
                c.push_str(&c.clone());
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);

                let c = &color[1..2];
                let mut c = c.to_string();
                c.push_str(&c.clone());
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);

                let c = &color[2..3];
                let mut c = c.to_string();
                c.push_str(&c.clone());
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);
            } else if color.len() == 6 {
                let c = &color[0..2];
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);
                let c = &color[2..4];
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);
                let c = &color[4..6];
                let z = u8::from_str_radix(&c, 16).unwrap();
                colors.push(z);
            } else {
                panic!("invalid color");
            }

            (colors[0], colors[1], colors[2], 1.0)
        } else {
            raw_background_color.retain(|c| {
                c == ','
                    || c == '.'
                    || c == '1'
                    || c == '0'
                    || c == '2'
                    || c == '3'
                    || c == '4'
                    || c == '5'
                    || c == '6'
                    || c == '7'
                    || c == '8'
                    || c == '9'
            });

            let colors = &raw_background_color;
            let colors: Vec<&str> = colors.split(",").collect();
            let colors = (
                colors[0].parse::<u8>().unwrap(),
                colors[1].parse::<u8>().unwrap(),
                colors[2].parse::<u8>().unwrap(),
                colors[3].parse::<f32>().unwrap(),
            );
            colors
        };
        return Some(Color::from_rgba8(colors.0, colors.1, colors.2, colors.3));
    }

    None
}
