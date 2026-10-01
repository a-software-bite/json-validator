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

/// Validates JSON string and returns detailed result.
/// This function is decoupled from any UI framework for easy testing.
pub fn validate_json(input: &str, kind: ParserKind) -> ValidationResult {
    if input.trim().is_empty() {
        return ValidationResult::invalid("Input is empty".to_string(), None, None);
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

    #[test]
    fn test_valid_object() {
        let result = validate_json(r#"{"key": "value"}"#, ParserKind::Fast);
        assert!(result.is_valid);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_valid_array() {
        let result = validate_json(r#"[1, 2, 3]"#, ParserKind::Fast);
        assert!(result.is_valid);
    }

    #[test]
    fn test_valid_nested() {
        let result = validate_json(r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#, ParserKind::Fast);
        assert!(result.is_valid);
    }

    #[test]
    fn test_valid_primitives() {
        assert!(validate_json("42", ParserKind::Fast).is_valid);
        assert!(validate_json("true", ParserKind::Fast).is_valid);
        assert!(validate_json("false", ParserKind::Fast).is_valid);
        assert!(validate_json("null", ParserKind::Fast).is_valid);
        assert!(validate_json(r#""hello""#, ParserKind::Fast).is_valid);
    }

    #[test]
    fn test_invalid_missing_quote() {
        let result = validate_json(r#"{"key: "value"}"#, ParserKind::Fast);
        assert!(!result.is_valid);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_invalid_trailing_comma() {
        let result = validate_json(r#"{"key": "value",}"#, ParserKind::Fast);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_invalid_single_quotes() {
        let result = validate_json(r#"{'key': 'value'}"#, ParserKind::Fast);
        assert!(!result.is_valid);
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
        assert!(result.error_line.is_some());
        assert!(result.error_column.is_some());
    }
}
