use anyhow::{Context, Result};
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Query, Tree};
use tree_sitter_mustache_json5::LANGUAGE;

use crate::config::Config;

pub struct MustacheJson5Formatter {
    parser: Parser,
    config: Config,
    indent_query: Query,
}

impl MustacheJson5Formatter {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let language = LANGUAGE.into();
        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .context("Failed to set language for parser")?;

        // Load indentation queries - embedded basic rules for now
        let indent_query_source = r#"
; Increase indent for JSON5 structures
(object "{" @indent.begin)
(object "}" @indent.end)

(array "[" @indent.begin)
(array "]" @indent.end)

; Increase indent for Mustache sections
(mustache_section
  (mustache_section_begin) @indent.begin)
(mustache_section
  (mustache_section_end) @indent.end)

(mustache_inverted_section
  (mustache_inverted_section_begin) @indent.begin)
(mustache_inverted_section
  (mustache_inverted_section_end) @indent.end)

; Align object members
(object
  (member) @indent.align)

; Align array elements
(array
  (_) @indent.align)
"#;
        let indent_query = Query::new(&language, indent_query_source)
            .context("Failed to parse indentation query")?;

        Ok(Self {
            parser,
            config,
            indent_query,
        })
    }

    pub fn format(&mut self, source: &str) -> Result<String> {
        let tree = self
            .parser
            .parse(source, None)
            .context("Failed to parse source code")?;

        if tree.root_node().has_error() {
            return Err(anyhow::anyhow!("Syntax error in source code"));
        }

        let formatted = self.format_tree(&tree, source)?;
        Ok(formatted)
    }

    fn format_tree(&self, tree: &Tree, source: &str) -> Result<String> {
        let root_node = tree.root_node();
        let mut formatter = NodeFormatter::new(&self.config, source, &self.indent_query);
        formatter.format_node(root_node, 0)
    }
}

struct NodeFormatter<'a> {
    config: &'a Config,
    source: &'a str,
    _indent_query: &'a Query,
    _lines: Vec<&'a str>,
}

impl<'a> NodeFormatter<'a> {
    fn new(config: &'a Config, source: &'a str, indent_query: &'a Query) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        Self {
            config,
            source,
            _indent_query: indent_query,
            _lines: lines,
        }
    }

    fn format_node(&mut self, node: Node, base_indent: usize) -> Result<String> {
        let mut result = String::new();
        let current_indent = base_indent;

        // Get indentation hints from tree-sitter queries
        let _indent_changes = self.get_indent_changes(node);

        match node.kind() {
            "template_document" => {
                for child in node.children(&mut node.walk()) {
                    if !child.is_named() {
                        continue;
                    }
                    result.push_str(&self.format_node(child, current_indent)?);
                }
            }
            "object" => {
                result.push_str(&self.format_object(node, current_indent)?);
            }
            "array" => {
                result.push_str(&self.format_array(node, current_indent)?);
            }
            "mustache_section" => {
                result.push_str(&self.format_mustache_section(node, current_indent)?);
            }
            "mustache_inverted_section" => {
                result.push_str(&self.format_mustache_inverted_section(node, current_indent)?);
            }
            "mustache_interpolation" => {
                result.push_str(&self.format_mustache_interpolation(node)?);
            }
            "mustache_unescaped" => {
                result.push_str(&self.format_mustache_unescaped(node)?);
            }
            "mustache_partial" => {
                result.push_str(&self.format_mustache_partial(node)?);
            }
            "mustache_comment" => {
                result.push_str(&self.format_mustache_comment(node, current_indent)?);
            }
            "comment" => {
                result.push_str(&self.format_json5_comment(node, current_indent)?);
            }
            _ => {
                // For other nodes, preserve original content with proper indentation
                result.push_str(&self.get_node_text(node));
            }
        }

        Ok(result)
    }

    fn format_object(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();
        let mut cursor = node.walk();
        let mut is_first_member = true;
        let inner_indent = indent + 1;

        for child in node.children(&mut cursor) {
            match child.kind() {
                "{" => {
                    result.push('{');
                    // Check if object is multiline
                    if self.should_format_multiline_object(node) {
                        result.push('\n');
                    }
                }
                "}" => {
                    if self.should_format_multiline_object(node) {
                        result.push('\n');
                        result.push_str(&self.indent_string(indent));
                    }
                    result.push('}');
                }
                "member" => {
                    if self.should_format_multiline_object(node) {
                        if !is_first_member {
                            result.push(',');
                            result.push('\n');
                        }
                        result.push_str(&self.indent_string(inner_indent));
                        result.push_str(&self.format_object_member(child, inner_indent)?);
                        is_first_member = false;
                    } else {
                        if !is_first_member {
                            result.push_str(", ");
                        }
                        result.push_str(&self.format_object_member(child, inner_indent)?);
                        is_first_member = false;
                    }
                }
                "," => {
                    // Commas are handled in member formatting
                }
                _ if child.is_named() => {
                    result.push_str(&self.format_node(child, inner_indent)?);
                }
                _ => {
                    // Handle whitespace and other unnamed nodes
                    if !self.should_format_multiline_object(node) {
                        let text = self.get_node_text(child);
                        if !text.trim().is_empty() {
                            result.push_str(text);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn format_array(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();
        let mut cursor = node.walk();
        let mut is_first_element = true;
        let inner_indent = indent + 1;

        for child in node.children(&mut cursor) {
            match child.kind() {
                "[" => {
                    result.push('[');
                    if self.should_format_multiline_array(node) {
                        result.push('\n');
                    }
                }
                "]" => {
                    if self.should_format_multiline_array(node) {
                        result.push('\n');
                        result.push_str(&self.indent_string(indent));
                    }
                    result.push(']');
                }
                "," => {
                    // Commas are handled in element formatting
                }
                _ if child.is_named() => {
                    if self.should_format_multiline_array(node) {
                        if !is_first_element {
                            result.push(',');
                            result.push('\n');
                        }
                        result.push_str(&self.indent_string(inner_indent));
                        result.push_str(&self.format_node(child, inner_indent)?);
                        is_first_element = false;
                    } else {
                        if !is_first_element {
                            result.push_str(", ");
                        }
                        result.push_str(&self.format_node(child, inner_indent)?);
                        is_first_element = false;
                    }
                }
                _ => {
                    // Handle whitespace and other unnamed nodes
                    if !self.should_format_multiline_array(node) {
                        let text = self.get_node_text(child);
                        if !text.trim().is_empty() {
                            result.push_str(text);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn format_mustache_section(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();
        let inner_indent = match self.config.mustache_indent_style {
            crate::config::MustacheIndentStyle::Block => indent + 1,
            crate::config::MustacheIndentStyle::Minimal => indent,
            crate::config::MustacheIndentStyle::Preserve => indent,
        };

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "mustache_section_begin" => {
                    result.push_str(&self.indent_string(indent));
                    result.push_str(&self.format_mustache_delimiter(child)?);
                    result.push('\n');
                }
                "mustache_section_end" => {
                    result.push_str(&self.indent_string(indent));
                    result.push_str(&self.format_mustache_delimiter(child)?);
                }
                "template_content" => {
                    let content = self.format_node(child, inner_indent)?;
                    if !content.trim().is_empty() {
                        result.push_str(&content);
                        if !content.ends_with('\n') {
                            result.push('\n');
                        }
                    }
                }
                _ if child.is_named() => {
                    result.push_str(&self.format_node(child, inner_indent)?);
                }
                _ => {
                    // Handle whitespace between sections
                    let text = self.get_node_text(child);
                    if text.contains('\n') {
                        result.push('\n');
                    }
                }
            }
        }

        Ok(result)
    }

    fn format_mustache_inverted_section(&mut self, node: Node, indent: usize) -> Result<String> {
        // Similar to regular sections but with ^ operator
        self.format_mustache_section(node, indent)
    }

    fn format_mustache_interpolation(&mut self, node: Node) -> Result<String> {
        self.format_mustache_delimiter(node)
    }

    fn format_mustache_unescaped(&mut self, node: Node) -> Result<String> {
        self.format_mustache_delimiter(node)
    }

    fn format_mustache_partial(&mut self, node: Node) -> Result<String> {
        self.format_mustache_delimiter(node)
    }

    fn format_mustache_delimiter(&mut self, node: Node) -> Result<String> {
        let mut result = String::new();
        let spacing = &self.config.mustache_spacing;

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "{{" => {
                    result.push_str("{{");
                    if spacing.after_open {
                        result.push(' ');
                    }
                }
                "}}" => {
                    if spacing.before_close {
                        result.push(' ');
                    }
                    result.push_str("}}");
                }
                "#" | "^" | "/" | "!" | ">" | "&" => {
                    if spacing.around_operators && !result.ends_with(' ') {
                        result.push(' ');
                    }
                    result.push_str(self.get_node_text(child));
                    if spacing.around_operators {
                        result.push(' ');
                    }
                }
                _ => {
                    let text = self.get_node_text(child).trim();
                    if !text.is_empty() {
                        result.push_str(text);
                    }
                }
            }
        }

        Ok(result)
    }

    fn format_object_member(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                ":" => {
                    result.push_str(": ");
                }
                _ if child.is_named() => {
                    result.push_str(&self.format_node(child, indent)?);
                }
                _ => {
                    let text = self.get_node_text(child);
                    if !text.trim().is_empty() {
                        result.push_str(text);
                    }
                }
            }
        }

        Ok(result)
    }

    fn format_mustache_comment(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();
        result.push_str(&self.indent_string(indent));
        result.push_str(&self.get_node_text(node));
        Ok(result)
    }

    fn format_json5_comment(&mut self, node: Node, indent: usize) -> Result<String> {
        let mut result = String::new();
        let text = self.get_node_text(node);

        if text.starts_with("//") {
            result.push_str(&self.indent_string(indent));
            result.push_str(text);
        } else if text.starts_with("/*") {
            // Handle multiline comments
            result.push_str(&self.indent_string(indent));
            result.push_str(text);
        }

        Ok(result)
    }

    fn should_format_multiline_object(&self, node: Node) -> bool {
        // Check if object spans multiple lines or has more than 2 members
        let member_count = node
            .children(&mut node.walk())
            .filter(|child| child.kind() == "member")
            .count();

        member_count > 2 || self.node_spans_multiple_lines(node)
    }

    fn should_format_multiline_array(&self, node: Node) -> bool {
        // Check if array spans multiple lines or has more than 3 elements
        let element_count = node
            .children(&mut node.walk())
            .filter(|child| child.is_named() && child.kind() != "[" && child.kind() != "]")
            .count();

        element_count > 3 || self.node_spans_multiple_lines(node)
    }

    fn node_spans_multiple_lines(&self, node: Node) -> bool {
        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        start_line != end_line
    }

    fn get_indent_changes(&self, _node: Node) -> HashMap<String, i32> {
        // This would use tree-sitter queries to determine indentation changes
        // For now, return empty map - we'll implement query-based indentation later
        HashMap::new()
    }

    fn indent_string(&self, level: usize) -> String {
        self.config.indent_string().repeat(level)
    }

    fn get_node_text(&self, node: Node) -> &str {
        &self.source[node.byte_range()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_creation() {
        let config = Config::default();
        let formatter = MustacheJson5Formatter::new(config);
        assert!(formatter.is_ok());
    }

    #[test]
    fn test_simple_json5_formatting() {
        let config = Config::default();
        let mut formatter = MustacheJson5Formatter::new(config).unwrap();

        let input = r#"{"name":"test","value":123}"#;
        let result = formatter.format(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mustache_section_formatting() {
        let config = Config::default();
        let mut formatter = MustacheJson5Formatter::new(config).unwrap();

        let input = r#"{{#users}}
{{name}}
{{/users}}"#;
        let result = formatter.format(input);
        assert!(result.is_ok());
    }
}
