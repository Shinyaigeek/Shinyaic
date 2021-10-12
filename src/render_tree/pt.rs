pub fn fix_unit_to_px(value: String, window_height: f32) -> Option<f32> {
    let value = if value.starts_with(".") {
        let mut v = String::from("0");
        v.push_str(&value);
        v
    } else {
        value
    };
    if value.ends_with("px") {
        let str_value = value.strip_suffix("px").unwrap();
        return Some(str_value.parse::<f32>().unwrap());
    }

    if value.ends_with("em") {
        // TODO
        let str_value = value.strip_suffix("em").unwrap();
        return Some(str_value.parse::<f32>().unwrap() * 18.0);
    }

    if value.ends_with("vh") {
        // TODO
        let str_value = value.strip_suffix("vh").unwrap();
        let vh = str_value.parse::<f32>().unwrap();
        return Some(vh * window_height);
    }

    if value.ends_with("vw") {
        // TODO
        let str_value = value.strip_suffix("vw").unwrap();
        let vh = str_value.parse::<f32>().unwrap();
        return Some(vh * window_height);
    }

    println!("value: {:?}", value);

    Some(value.parse::<f32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_unit_to_px() {
        assert_eq!(fix_unit_to_px("10px".to_string(), 0.0), Some(10.0));
    }

    #[test]
    fn test_fix_unit_to_px_without_px() {
        assert_eq!(fix_unit_to_px("10".to_string(), 0.0), Some(10.0));
    }
}
