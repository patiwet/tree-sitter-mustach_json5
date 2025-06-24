//! A formatter for Mustache JSON5 templates using tree-sitter
//!
//! This crate provides a formatter that can intelligently format files containing
//! a mix of JSON5 and Mustache template syntax. It leverages the full tree-sitter
//! grammar context to provide high-quality formatting while preserving the semantic
//! meaning of templates.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! use mustache_json5_fmt::{Config, MustacheJson5Formatter};
//!
//! let config = Config::default();
//! let mut formatter = MustacheJson5Formatter::new(config)?;
//!
//! let input = r#"{"name":"{{user.name}}","items":[{{#items}}"{{.}}"{{#unless @last}},{{/unless}}{{/items}}]}"#;
//! let formatted = formatter.format(input)?;
//!
//! println!("{}", formatted);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! With custom configuration:
//!
//! ```rust
//! use mustache_json5_fmt::{Config, MustacheJson5Formatter};
//!
//! let mut config = Config::default();
//! config.indent_size = 4;
//! config.use_tabs = false;
//!
//! let mut formatter = MustacheJson5Formatter::new(config)?;
//! let formatted = formatter.format(r#"{"key":"value"}"#)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod config;
pub mod formatter;

pub use config::{
    CommentHandling, Config, MustacheIndentStyle, MustacheSpacing, QuoteStyle, TrailingCommaStyle,
};
pub use formatter::MustacheJson5Formatter;

/// Result type used throughout the crate
pub type Result<T> = anyhow::Result<T>;

/// Format a string of mustache_json5 content with default configuration
///
/// This is a convenience function for simple formatting needs.
/// For more control over formatting options, use `MustacheJson5Formatter` directly.
///
/// # Arguments
///
/// * `input` - The mustache_json5 content to format
///
/// # Returns
///
/// The formatted content as a string, or an error if formatting fails
///
/// # Examples
///
/// ```rust
/// use mustache_json5_fmt::format_string;
///
/// let input = r#"{"name":"test","value":123}"#;
/// let formatted = format_string(input)?;
/// assert!(formatted.contains("  \"name\": \"test\""));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn format_string(input: &str) -> Result<String> {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config)?;
    formatter.format(input)
}

/// Format a string with custom configuration
///
/// # Arguments
///
/// * `input` - The mustache_json5 content to format
/// * `config` - The configuration to use for formatting
///
/// # Returns
///
/// The formatted content as a string, or an error if formatting fails
///
/// # Examples
///
/// ```rust
/// use mustache_json5_fmt::{format_string_with_config, Config};
///
/// let mut config = Config::default();
/// config.indent_size = 4;
///
/// let input = r#"{"name":"test"}"#;
/// let formatted = format_string_with_config(input, config)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn format_string_with_config(input: &str, config: Config) -> Result<String> {
    let mut formatter = MustacheJson5Formatter::new(config)?;
    formatter.format(input)
}

/// Check if a string is properly formatted according to the given configuration
///
/// # Arguments
///
/// * `input` - The mustache_json5 content to check
/// * `config` - The configuration to use for checking
///
/// # Returns
///
/// `true` if the input is already properly formatted, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use mustache_json5_fmt::{is_formatted, Config};
///
/// let config = Config::default();
/// let properly_formatted = r#"{
///   "name": "test",
///   "value": 123
/// }"#;
///
/// assert!(is_formatted(properly_formatted, &config)?);
///
/// let poorly_formatted = r#"{"name":"test","value":123}"#;
/// assert!(!is_formatted(poorly_formatted, &config)?);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn is_formatted(input: &str, config: &Config) -> Result<bool> {
    let formatted = format_string_with_config(input, config.clone())?;
    Ok(input == formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_string() {
        let input = r#"{"name":"test","value":123}"#;
        let result = format_string(input);
        assert!(result.is_ok());

        let formatted = result.unwrap();
        assert!(formatted.contains("  \"name\": \"test\""));
        assert!(formatted.contains("  \"value\": 123"));
    }

    #[test]
    fn test_format_string_with_config() {
        let mut config = Config::default();
        config.indent_size = 4;

        let input = r#"{"name":"test"}"#;
        let result = format_string_with_config(input, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_formatted() {
        let config = Config::default();

        let properly_formatted = r#"{
  "name": "test"
}"#;
        assert!(is_formatted(properly_formatted, &config).unwrap());

        let poorly_formatted = r#"{"name":"test"}"#;
        assert!(!is_formatted(poorly_formatted, &config).unwrap());
    }

    #[test]
    fn test_mustache_formatting() {
        let input = r#"{{#users}}{{name}}{{/users}}"#;
        let result = format_string(input);
        assert!(result.is_ok());

        let formatted = result.unwrap();
        assert!(formatted.contains("{{#users}}"));
        assert!(formatted.contains("{{/users}}"));
    }

    #[test]
    fn test_mixed_content() {
        let input =
            r#"{"users":[{{#each users}}"{{name}}"{{#unless @last}},{{/unless}}{{/each}}]}"#;
        let result = format_string(input);
        assert!(result.is_ok());

        let formatted = result.unwrap();
        assert!(formatted.contains("\"users\": ["));
        assert!(formatted.contains("{{#each users}}"));
    }

    #[test]
    fn test_error_handling() {
        let invalid_input = r#"{"name": }"#; // Invalid JSON5
        let result = format_string(invalid_input);
        assert!(result.is_err());
    }
}
