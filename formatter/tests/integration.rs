use mustache_json5_fmt::{Config, MustacheJson5Formatter};
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::TempDir;

mod fixtures;
use fixtures::*;

#[test]
fn test_basic_json5_formatting() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in BASIC_JSON5_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_mustache_formatting() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in MUSTACHE_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_mixed_content_formatting() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in MIXED_CONTENT_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_comment_preservation() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in COMMENT_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_edge_cases() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in EDGE_CASE_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_real_world_examples() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in REAL_WORLD_TESTS {
        let result = formatter
            .format(test_case.input)
            .expect(&format!("Failed to format test case: {}", test_case.name));

        assert_eq!(
            result.trim(),
            test_case.expected.trim(),
            "Test case '{}' failed: {}",
            test_case.name,
            test_case.description
        );
    }
}

#[test]
fn test_configuration_options() {
    // Test with tabs
    let mut config = Config::default();
    config.use_tabs = true;
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    let input = r#"{"name":"test","value":123}"#;
    let result = formatter.format(input).expect("Failed to format with tabs");
    assert!(result.contains('\t'), "Expected tabs in output");

    // Test with different indent size
    let mut config = Config::default();
    config.indent_size = 4;
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    let result = formatter
        .format(input)
        .expect("Failed to format with 4-space indent");
    let lines: Vec<&str> = result.lines().collect();
    if lines.len() > 1 {
        // Check that indented lines use 4 spaces
        for line in lines.iter().skip(1) {
            if line.trim() != "" && line.starts_with(' ') {
                let leading_spaces = line.len() - line.trim_start().len();
                assert!(
                    leading_spaces % 4 == 0,
                    "Expected multiple of 4 spaces, got {}",
                    leading_spaces
                );
            }
        }
    }
}

#[test]
fn test_idempotency() {
    // Test that formatting the same content twice produces the same result
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    for test_case in all_test_cases() {
        let first_pass = formatter
            .format(test_case.input)
            .expect(&format!("Failed first format pass for: {}", test_case.name));

        let second_pass = formatter.format(&first_pass).expect(&format!(
            "Failed second format pass for: {}",
            test_case.name
        ));

        assert_eq!(
            first_pass, second_pass,
            "Formatting is not idempotent for test case: {}",
            test_case.name
        );
    }
}

#[test]
fn test_syntax_error_handling() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    let invalid_inputs = vec![
        r#"{"name": }"#,             // Invalid JSON5
        r#"{{#unclosed"#,            // Unclosed mustache
        r#"{{#section}}{{/other}}"#, // Mismatched section tags
        r#"{"name": "{{}"#,          // Mixed syntax error
    ];

    for input in invalid_inputs {
        let result = formatter.format(input);
        assert!(
            result.is_err(),
            "Expected error for invalid input: {}",
            input
        );
    }
}

#[test]
fn test_file_operations() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.mustache_json5");

    let input = r#"{"name":"test","nested":{"value":123}}"#;
    let expected = r#"{
  "name": "test",
  "nested": {
    "value": 123
  }
}"#;

    fs::write(&test_file, input).expect("Failed to write test file");

    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    let content = fs::read_to_string(&test_file).expect("Failed to read test file");
    let formatted = formatter
        .format(&content)
        .expect("Failed to format file content");

    assert_eq!(formatted.trim(), expected.trim());
}

#[test]
fn test_performance_large_file() {
    use std::time::Instant;

    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    // Generate a large test file
    let mut large_content = String::new();
    large_content.push_str("{\n");
    large_content.push_str("  \"users\": [\n");

    for i in 0..1000 {
        large_content.push_str(&format!(
            "    {{{{#users}}}}\n    {{\n      \"id\": {i},\n      \"name\": \"{{{{name}}}}\",\n      \"email\": \"{{{{email}}}}\"\n    }}{{{{#unless @last}}}},{{{{/unless}}}}\n    {{{{/users}}}}"
        ));
        if i < 999 {
            large_content.push_str(",\n");
        }
    }

    large_content.push_str("\n  ]\n}");

    let start = Instant::now();
    let result = formatter.format(&large_content);
    let duration = start.elapsed();

    assert!(result.is_ok(), "Failed to format large file");
    assert!(
        duration.as_millis() < 1000,
        "Formatting took too long: {:?}",
        duration
    );
}

#[test]
fn test_whitespace_handling() {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config).expect("Failed to create formatter");

    let test_cases = vec![
        // Test various whitespace scenarios
        ("   {\"name\":\"test\"}   ", "{\n  \"name\": \"test\"\n}"),
        (
            "{\n\n\n  \"name\":  \"test\"\n\n\n}",
            "{\n  \"name\": \"test\"\n}",
        ),
        (
            "{{#users}}\n\n\n{{name}}\n\n\n{{/users}}",
            "{{#users}}\n  {{name}}\n{{/users}}",
        ),
    ];

    for (input, expected) in test_cases {
        let result = formatter
            .format(input)
            .expect("Failed to format whitespace test");
        assert_eq!(result.trim(), expected.trim());
    }
}
