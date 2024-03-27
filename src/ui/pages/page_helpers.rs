use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let text_with_space = format!(" {}", text);
    if width >= text_with_space.len() {
        return format!("{:width$}", text_with_space, width = width);
    }
    let formatted_text = text_with_space
        .as_str()
        .truncate_ellipse(if width > 3 { width - 3 } else { width })
        .to_string();
    if formatted_text.len() > width {
        return formatted_text[(formatted_text.len() - width)..].to_string();
    }
    formatted_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
