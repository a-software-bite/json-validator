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
pub fn validate_json(input: &str) -> ValidationResult {
    if input.trim().is_empty() {
        return ValidationResult::invalid(
            "Input is empty".to_string(),
            None,
            None,
        );
    }

    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(_) => ValidationResult::valid(),
        Err(e) => ValidationResult::invalid(
            e.to_string(),
            Some(e.line()),
            Some(e.column()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_object() {
        let result = validate_json(r#"{"key": "value"}"#);
        assert!(result.is_valid);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_valid_array() {
        let result = validate_json(r#"[1, 2, 3]"#);
        assert!(result.is_valid);
    }

    #[test]
    fn test_valid_nested() {
        let result = validate_json(r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#);
        assert!(result.is_valid);
    }

    #[test]
    fn test_valid_primitives() {
        assert!(validate_json("42").is_valid);
        assert!(validate_json("true").is_valid);
        assert!(validate_json("false").is_valid);
        assert!(validate_json("null").is_valid);
        assert!(validate_json(r#""hello""#).is_valid);
    }

    #[test]
    fn test_invalid_missing_quote() {
        let result = validate_json(r#"{"key: "value"}"#);
        assert!(!result.is_valid);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_invalid_trailing_comma() {
        let result = validate_json(r#"{"key": "value",}"#);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_invalid_single_quotes() {
        let result = validate_json(r#"{'key': 'value'}"#);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_empty_input() {
        let result = validate_json("");
        assert!(!result.is_valid);
        assert_eq!(result.error_message, Some("Input is empty".to_string()));
    }

    #[test]
    fn test_whitespace_only() {
        let result = validate_json("   \n\t  ");
        assert!(!result.is_valid);
    }

    #[test]
    fn test_error_location() {
        let result = validate_json("{\n  \"key\": value\n}");
        assert!(!result.is_valid);
        assert!(result.error_line.is_some());
        assert!(result.error_column.is_some());
    }
}
