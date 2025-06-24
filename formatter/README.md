# Mustache JSON5 Formatter

A high-performance formatter for files containing a mix of Mustache templates and JSON5 syntax, built with tree-sitter for accurate parsing and formatting.

## Features

- **Intelligent Formatting**: Understands both JSON5 and Mustache syntax simultaneously
- **Tree-sitter Powered**: Leverages the full grammar context for accurate formatting
- **Configurable**: Flexible configuration options for different coding styles
- **Fast**: Rust-based with excellent performance characteristics
- **CLI Tool**: Easy-to-use command-line interface
- **Library**: Can be used as a Rust library in other projects

## Installation

### From Source

```bash
git clone https://github.com/yourusername/tree-sitter-mustache_json5.git
cd tree-sitter-mustache_json5/formatter
cargo install --path .
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/yourusername/tree-sitter-mustache_json5/releases)

## Usage

### Command Line

```bash
# Format a single file
mustache-json5-fmt file.mustache_json5

# Format multiple files
mustache-json5-fmt **/*.mustache_json5

# Format and write back to files
mustache-json5-fmt --write file.mustache_json5

# Check if files are formatted (CI/CD)
mustache-json5-fmt --check **/*.mustache_json5

# Read from stdin
cat file.mustache_json5 | mustache-json5-fmt --stdin

# Custom configuration
mustache-json5-fmt --indent-size 4 --use-tabs file.mustache_json5
```

### Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
mustache-json5-fmt = "0.1"
```

Basic usage:

```rust
use mustache_json5_fmt::{Config, MustacheJson5Formatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let mut formatter = MustacheJson5Formatter::new(config)?;
    
    let input = r#"{"name":"{{user.name}}","items":[{{#items}}"{{.}}"{{#unless @last}},{{/unless}}{{/items}}]}"#;
    let formatted = formatter.format(input)?;
    
    println!("{}", formatted);
    Ok(())
}
```

## Configuration

Create a `.mustache-json5-fmt.json` file in your project root:

```json
{
  "indent_size": 2,
  "use_tabs": false,
  "max_line_length": 100,
  "preserve_empty_lines": true,
  "trailing_commas": "preserve",
  "quote_style": "preserve",
  "mustache_spacing": {
    "after_open": false,
    "before_close": false,
    "around_operators": false
  },
  "mustache_indent_style": "block",
  "comment_handling": {
    "preserve_formatting": true,
    "normalize_spacing": false
  }
}
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `indent_size` | number | `2` | Number of spaces for indentation |
| `use_tabs` | boolean | `false` | Use tabs instead of spaces |
| `max_line_length` | number | `0` | Maximum line length (0 = no limit) |
| `preserve_empty_lines` | boolean | `true` | Keep empty lines between sections |
| `trailing_commas` | string | `"preserve"` | `"never"`, `"always"`, or `"preserve"` |
| `quote_style` | string | `"preserve"` | `"double"`, `"single"`, or `"preserve"` |
| `mustache_spacing.after_open` | boolean | `false` | Space after `{{` |
| `mustache_spacing.before_close` | boolean | `false` | Space before `}}` |
| `mustache_spacing.around_operators` | boolean | `false` | Space around `#`, `^`, etc. |
| `mustache_indent_style` | string | `"block"` | `"block"`, `"preserve"`, or `"minimal"` |

## Examples

### Basic JSON5 Formatting

**Input:**
```json5
{"name":"John","age":30,"city":"New York"}
```

**Output:**
```json5
{
  "name": "John",
  "age": 30,
  "city": "New York"
}
```

### Mustache Template Formatting

**Input:**
```mustache
{{#users}}
{{name}} - {{email}}
{{/users}}
```

**Output:**
```mustache
{{#users}}
  {{name}} - {{email}}
{{/users}}
```

### Mixed Content (The Complex Case)

**Input:**
```json5
{"users":[
{{#each users}}
{"id":{{id}},"name":"{{name}}","email":"{{email}}"{{#if profile}},"profile":{"avatar":"{{profile.avatar}}","bio":"{{profile.bio}}"}}{{/if}}{{#unless @last}},{{/unless}}
{{/each}}
]}
```

**Output:**
```json5
{
  "users": [
    {{#each users}}
    {
      "id": {{id}},
      "name": "{{name}}",
      "email": "{{email}}"{{#if profile}},
      "profile": {
        "avatar": "{{profile.avatar}}",
        "bio": "{{profile.bio}}"
      }{{/if}}
    }{{#unless @last}},{{/unless}}
    {{/each}}
  ]
}
```

## Editor Integration

### Zed Editor

The formatter is designed to work seamlessly with Zed editor through the `zed-mustache-json5` extension.

### VS Code

Use the `mustache-json5-fmt` formatter with the "Format Document" command or format-on-save.

### Other Editors

Any editor with tree-sitter support can use this formatter. See [Editor Integration Guide](docs/editor-integration.md) for details.

## Performance

The formatter is designed for speed and can handle large files efficiently:

- **Parsing Speed**: ~8,000 bytes/ms
- **Memory Usage**: Minimal overhead
- **Incremental**: Only re-formats changed regions (when used as library)

## Development

### Building

```bash
git clone https://github.com/yourusername/tree-sitter-mustache_json5.git
cd tree-sitter-mustache_json5/formatter
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Running Benchmarks

```bash
cargo bench
```

## FAQ

**Q: Why not just use Prettier?**
A: Prettier doesn't understand the complex interaction between Mustache templates and JSON5 syntax. This formatter uses the full tree-sitter grammar to make intelligent formatting decisions.

**Q: Does it support all Mustache features?**
A: Yes, it supports the complete Mustache specification including sections, inverted sections, partials, comments, and helper functions.

**Q: Can I use it in CI/CD?**
A: Yes! Use the `--check` flag to verify formatting in your CI pipeline.

**Q: What about JavaScript/TypeScript in templates?**
A: The formatter handles embedded languages through tree-sitter's injection system, providing syntax-aware formatting for JavaScript, CSS, HTML, and more.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Related Projects

- [tree-sitter-mustache_json5](../) - The tree-sitter grammar this formatter is based on
- [zed-mustache-json5](https://github.com/yourusername/zed-mustache-json5) - Zed editor extension
- [vscode-mustache-json5](https://github.com/yourusername/vscode-mustache-json5) - VS Code extension