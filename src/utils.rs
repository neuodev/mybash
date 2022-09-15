use regex::Regex;

use crate::regex::{RE_COMMENT, RE_INPUT_FUNC};

pub fn remove_comments<'a>(s: &'a str) -> String {
    let re = Regex::new(RE_COMMENT).unwrap();

    if let Some(caps) = re.captures(s) {
        s.replace(&caps[0], "")
            .replace("\\#", "#")
            .trim()
            .to_string()
    } else {
        s.trim().replace("\\#", "#").to_string()
    }
}

pub fn is_input_fn(s: &str) -> bool {
    let re = Regex::new(RE_INPUT_FUNC).unwrap();
    re.is_match(s.trim())
}

#[cfg(test)]
mod test {
    use super::remove_comments;

    #[test]
    fn should_ignore_line_comment() {
        let expr = "# I am a line comment";
        assert_eq!(remove_comments(expr), "".to_string());
    }

    #[test]
    fn should_remove_inline_comment() {
        let expr = "echo \"Hello, World\" # I am an inline comment";
        assert_eq!(remove_comments(expr), "echo \"Hello, World\"".to_string());
    }

    #[test]
    fn should_ignore_scaped_comments() {
        let expr = r#"name: string = "\#hash\#" # I am a comment"#;
        assert_eq!(
            remove_comments(expr),
            "name: string = \"#hash#\"".to_string()
        );
    }
}
