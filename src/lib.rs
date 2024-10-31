include!("bindings.rs");

#[cfg(test)]
mod test {
    use crate::Color;

    #[test]
    fn colors_in_screaming_cap_case() {
        assert_eq!(Color::CornflowerBlue.as_ref(), "CORNFLOWER_BLUE");
    }
} 