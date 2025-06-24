use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Number of spaces for indentation (when not using tabs)
    pub indent_size: usize,

    /// Width of tab characters for display purposes
    pub tab_width: usize,

    /// Use tabs instead of spaces for indentation
    pub use_tabs: bool,

    /// Maximum line length before wrapping (0 = no limit)
    pub max_line_length: usize,

    /// Preserve empty lines between sections
    pub preserve_empty_lines: bool,

    /// How to handle trailing commas in JSON5
    pub trailing_commas: TrailingCommaStyle,

    /// Quote style for JSON5 strings
    pub quote_style: QuoteStyle,

    /// Spacing around mustache operators
    pub mustache_spacing: MustacheSpacing,

    /// Indentation style for mustache sections
    pub mustache_indent_style: MustacheIndentStyle,

    /// How to handle comments
    pub comment_handling: CommentHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrailingCommaStyle {
    /// Never add trailing commas
    Never,
    /// Add trailing commas where valid in JSON5
    Always,
    /// Keep existing trailing commas, don't add new ones
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuoteStyle {
    /// Always use double quotes
    Double,
    /// Always use single quotes (where possible in JSON5)
    Single,
    /// Preserve existing quote style
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MustacheSpacing {
    /// Space after opening delimiter: {{ vs {{
    pub after_open: bool,
    /// Space before closing delimiter: }} vs }}
    pub before_close: bool,
    /// Space around operators: {{# vs {{ #
    pub around_operators: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MustacheIndentStyle {
    /// Indent mustache sections like code blocks
    Block,
    /// Preserve original indentation
    Preserve,
    /// Minimal indentation (content at same level as delimiter)
    Minimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentHandling {
    /// Preserve existing comment formatting
    pub preserve_formatting: bool,
    /// Normalize comment spacing
    pub normalize_spacing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent_size: 2,
            tab_width: 2,
            use_tabs: false,
            max_line_length: 0, // No limit by default
            preserve_empty_lines: true,
            trailing_commas: TrailingCommaStyle::Preserve,
            quote_style: QuoteStyle::Preserve,
            mustache_spacing: MustacheSpacing {
                after_open: false,
                before_close: false,
                around_operators: false,
            },
            mustache_indent_style: MustacheIndentStyle::Block,
            comment_handling: CommentHandling {
                preserve_formatting: true,
                normalize_spacing: false,
            },
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: Config =
            serde_json::from_str(&content).context("Failed to parse config file as JSON")?;

        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Load configuration with fallback chain:
    /// 1. Explicit config file path
    /// 2. .mustache-json5-fmt.json in current directory
    /// 3. .mustache-json5-fmt.json in parent directories (up to home)
    /// 4. Default configuration
    pub fn load_with_fallback(explicit_path: Option<&str>) -> Result<Self> {
        // Try explicit path first
        if let Some(path) = explicit_path {
            return Self::from_file(path);
        }

        // Try current directory
        let current_config = Path::new(".mustache-json5-fmt.json");
        if current_config.exists() {
            return Self::from_file(current_config);
        }

        // Try parent directories
        let mut current_dir = std::env::current_dir().context("Failed to get current directory")?;

        loop {
            current_dir = match current_dir.parent() {
                Some(parent) => parent.to_path_buf(),
                None => break,
            };

            let config_path = current_dir.join(".mustache-json5-fmt.json");
            if config_path.exists() {
                return Self::from_file(config_path);
            }

            // Stop at home directory
            if let Some(home_dir) = dirs::home_dir() {
                if current_dir == home_dir {
                    break;
                }
            }
        }

        // Return default configuration
        Ok(Self::default())
    }

    /// Get the indentation string based on configuration
    pub fn indent_string(&self) -> String {
        if self.use_tabs {
            "\t".to_string()
        } else {
            " ".repeat(self.indent_size)
        }
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        if self.indent_size == 0 {
            anyhow::bail!("indent_size must be greater than 0");
        }

        if self.tab_width == 0 {
            anyhow::bail!("tab_width must be greater than 0");
        }

        Ok(())
    }
}

impl Default for TrailingCommaStyle {
    fn default() -> Self {
        TrailingCommaStyle::Preserve
    }
}

impl Default for QuoteStyle {
    fn default() -> Self {
        QuoteStyle::Preserve
    }
}

impl Default for MustacheSpacing {
    fn default() -> Self {
        Self {
            after_open: false,
            before_close: false,
            around_operators: false,
        }
    }
}

impl Default for MustacheIndentStyle {
    fn default() -> Self {
        MustacheIndentStyle::Block
    }
}

impl Default for CommentHandling {
    fn default() -> Self {
        Self {
            preserve_formatting: true,
            normalize_spacing: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.indent_size, 2);
        assert!(!config.use_tabs);
        assert_eq!(config.indent_string(), "  ");
    }

    #[test]
    fn test_tab_config() {
        let mut config = Config::default();
        config.use_tabs = true;
        assert_eq!(config.indent_string(), "\t");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config.indent_size, deserialized.indent_size);
        assert_eq!(config.use_tabs, deserialized.use_tabs);
    }

    #[test]
    fn test_config_file_operations() {
        let config = Config::default();

        let mut temp_file = NamedTempFile::new().unwrap();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        temp_file.write_all(config_json.as_bytes()).unwrap();

        let loaded_config = Config::from_file(temp_file.path()).unwrap();
        assert_eq!(config.indent_size, loaded_config.indent_size);
        assert_eq!(config.use_tabs, loaded_config.use_tabs);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.validate().unwrap();

        config.indent_size = 0;
        assert!(config.validate().is_err());
    }
}
