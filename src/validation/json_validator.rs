use rust_json_parser::{parse_json, JsonParser};

/// Selects which of the crate's two public parsers runs the validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserKind {
    /// `parse_json`: single-pass byte scanner.
    Fast,
    /// `JsonParser`: tokenizer plus recursive descent.
    Slow,
}

impl ParserKind {
    pub fn parse<'a>(
        self,
        input: &'a str,
    ) -> rust_json_parser::JsonResult<rust_json_parser::JsonValue> {
        match self {
            ParserKind::Fast => parse_json(input),
            ParserKind::Slow => JsonParser::new(input).and_then(|mut parser| parser.parse()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub error_line: Option<usize>,
    pub error_column: Option<usize>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            error_line: None,
            error_column: None,
        }
    }

    pub fn invalid(message: String, line: Option<usize>, column: Option<usize>) -> Self {
        Self {
            is_valid: false,
            error_message: Some(message),
            error_line: line,
            error_column: column,
        }
    }
}

const MAX_INPUT_LEN: usize = 1_000_000;

/// Validates JSON string and returns detailed result.
/// This function is decoupled from any UI framework for easy testing.
pub fn validate_json(input: &str, kind: ParserKind) -> ValidationResult {
    if input.trim().is_empty() {
        return ValidationResult::invalid("Input is empty".to_string(), None, None);
    }

    if input.len() > MAX_INPUT_LEN {
        return ValidationResult::invalid(
            format!("Input exceeds maximum length of {MAX_INPUT_LEN} bytes"),
            None,
            None,
        );
    }

    match kind.parse(input) {
        Ok(_) => ValidationResult::valid(),
        Err(e) => {
            let (line, column) = e
                .line_column(input)
                .map(|(l, c)| (Some(l), Some(c)))
                .unwrap_or((None, None));
            ValidationResult::invalid(e.to_string(), line, column)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KINDS: [ParserKind; 2] = [ParserKind::Fast, ParserKind::Slow];

    #[test]
    fn test_valid_object() {
        for kind in KINDS {
            let result = validate_json(r#"{"key": "value"}"#, kind);
            assert!(result.is_valid, "{kind:?}");
            assert!(result.error_message.is_none(), "{kind:?}");
        }
    }

    #[test]
    fn test_valid_array() {
        for kind in KINDS {
            assert!(validate_json(r#"[1, 2, 3]"#, kind).is_valid, "{kind:?}");
        }
    }

    #[test]
    fn test_valid_nested() {
        for kind in KINDS {
            assert!(
                validate_json(r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#, kind).is_valid,
                "{kind:?}"
            );
        }
    }

    #[test]
    fn test_valid_primitives() {
        for kind in KINDS {
            assert!(validate_json("42", kind).is_valid, "{kind:?}");
            assert!(validate_json("true", kind).is_valid, "{kind:?}");
            assert!(validate_json("false", kind).is_valid, "{kind:?}");
            assert!(validate_json("null", kind).is_valid, "{kind:?}");
            assert!(validate_json(r#""hello""#, kind).is_valid, "{kind:?}");
        }
    }

    #[test]
    fn test_invalid_missing_quote() {
        for kind in KINDS {
            let result = validate_json(r#"{"key: "value"}"#, kind);
            assert!(!result.is_valid, "{kind:?}");
            // `value` is read as an invalid keyword at byte 8
            assert_eq!(result.error_line, Some(1), "{kind:?}");
            assert_eq!(result.error_column, Some(9), "{kind:?}");
        }
    }

    #[test]
    fn test_invalid_trailing_comma() {
        for kind in KINDS {
            let result = validate_json(r#"{"key": "value",}"#, kind);
            assert!(!result.is_valid, "{kind:?}");
            // the closing brace at byte 16 is unexpected
            assert_eq!(result.error_line, Some(1), "{kind:?}");
            assert_eq!(result.error_column, Some(17), "{kind:?}");
        }
    }

    #[test]
    fn test_invalid_single_quotes() {
        for kind in KINDS {
            let result = validate_json(r#"{'key': 'value'}"#, kind);
            assert!(!result.is_valid, "{kind:?}");
            // single quote is invalid punctuation at byte 1
            assert_eq!(result.error_line, Some(1), "{kind:?}");
            assert_eq!(result.error_column, Some(2), "{kind:?}");
        }
    }

    #[test]
    fn test_missing_comma_reports_key_position() {
        for kind in KINDS {
            let result = validate_json("{\n  \"key\": 42\n  \"key2\": true\n}", kind);
            assert!(!result.is_valid, "{kind:?}");
            // both parsers point at the second key, line 3 column 3
            assert_eq!(result.error_line, Some(3), "{kind:?}");
            assert_eq!(result.error_column, Some(3), "{kind:?}");
        }
    }

    #[test]
    fn test_empty_input() {
        let result = validate_json("", ParserKind::Fast);
        assert!(!result.is_valid);
        assert_eq!(result.error_message, Some("Input is empty".to_string()));
    }

    #[test]
    fn test_whitespace_only() {
        let result = validate_json("   \n\t  ", ParserKind::Fast);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_error_location() {
        let result = validate_json("{\n  \"key\": value\n}", ParserKind::Fast);
        assert!(!result.is_valid);
        // `value` is read as an invalid keyword at line 2, column 10
        assert_eq!(result.error_line, Some(2));
        assert_eq!(result.error_column, Some(10));
    }

    #[test]
    fn test_input_too_large() {
        let oversized = "x".repeat(MAX_INPUT_LEN + 1);
        let result = validate_json(&oversized, ParserKind::Fast);
        assert!(!result.is_valid);
        assert!(result
            .error_message
            .unwrap()
            .contains("maximum length"));
    }
}
