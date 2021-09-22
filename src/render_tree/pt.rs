pub fn fix_unit_to_px(value: String) -> Option<f32> {
    if value.ends_with("px") {
        let str_value = value.strip_suffix("px").unwrap();
        return Some(str_value.parse::<f32>().unwrap());
    }

    Some(value.parse::<f32>().unwrap())
}
