pub fn fix_unit_to_px(value: String) -> Option<f32> {
    if value.ends_with("px") {
        let str_value = value.strip_suffix("px").unwrap();
        return Some(str_value.parse::<f32>().unwrap());
    }

    if value.ends_with("em") {
        // TODO
        let str_value = value.strip_suffix("em").unwrap();
        return Some(str_value.parse::<f32>().unwrap() * 18.0);
    }

    Some(value.parse::<f32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_unit_to_px() {
        assert_eq!(fix_unit_to_px("10px".to_string()), Some(10.0));
    }

    #[test]
    fn test_fix_unit_to_px_without_px() {
        assert_eq!(fix_unit_to_px("10".to_string()), Some(10.0));
    }
}
