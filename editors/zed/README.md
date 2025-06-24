# Mustache JSON5 Extension for Zed

This extension provides comprehensive language support for Mustache JSON5 templates in the Zed editor, including syntax highlighting, intelligent formatting, and seamless integration with the `mustache-json5-fmt` formatter.

## Features

- **🎨 Syntax Highlighting**: Full syntax highlighting for mixed JSON5 and Mustache template syntax
- **⚡ Auto-formatting**: Intelligent formatting that preserves Mustache template structure while formatting JSON5
- **🔧 Bracket Matching**: Support for JSON5 brackets `{}`, `[]` and Mustache delimiters `{{}}`, `{{{}}}`
- **💬 Comment Support**: Line comments (`//`) and block comments (`/* */`)
- **📁 File Association**: Automatic activation for `.mustache_json5` and `.mjson5` files
- **🎯 Tree-sitter Integration**: Powered by a custom tree-sitter grammar for accurate parsing

## File Extensions

This extension automatically activates for files with the following extensions:
- `.mustache_json5`
- `.mjson5`

## Quick Start

1. **Install the Extension**: Copy this extension to your Zed extensions directory
2. **Install the Formatter**: Run the installation script or install manually
3. **Start Editing**: Open any `.mustache_json5` file and enjoy syntax highlighting and formatting

## Formatter Installation

The extension requires the `mustache-json5-fmt` formatter for formatting functionality.

### Option 1: Automatic Installation (Recommended)

```bash
# From the extension directory
./install-formatter.sh
```

This script will:
- Build the formatter from source
- Install it to `~/.local/bin/`
- Provide PATH configuration instructions

### Option 2: Manual Installation

1. **Clone and build**:
   ```bash
   git clone https://github.com/tree-sitter/tree-sitter-mustache_json5
   cd tree-sitter-mustache_json5/formatter
   cargo build --release
   ```

2. **Install globally**:
   ```bash
   # Option A: Install via cargo
   cargo install --path .
   
   # Option B: Copy to PATH
   cp target/release/mustache-json5-fmt ~/.local/bin/
   ```

3. **Verify installation**:
   ```bash
   mustache-json5-fmt --version
   ```

### Option 3: Download Release Binary

When available, download the latest release binary for your platform from the [releases page](https://github.com/tree-sitter/tree-sitter-mustache_json5/releases).

## Formatter Configuration

The formatter supports extensive configuration through command-line options:

### Basic Options
- `--indent-size N`: Set indentation size (default: 2)
- `--use-tabs`: Use tabs instead of spaces
- `--tab-width N`: Set tab width (default: 2)

### Advanced Options
- `--check`: Check if files are formatted (exit code 1 if not)
- `--write`: Format files in place
- `--stdin`: Read from stdin
- `--config PATH`: Use configuration file
- `--verbose`: Enable verbose output

### Configuration File

Create a `.mustache-json5-fmt.json` file in your project root:

```json
{
  "indent_size": 2,
  "use_tabs": false,
  "tab_width": 2,
  "quote_style": "Double",
  "trailing_comma_style": "Never",
  "mustache_indent_style": "Block",
  "mustache_spacing": {
    "around_delimiters": true,
    "inside_expressions": true
  }
}
```

## Usage in Zed

### Formatting
- **Format Document**: Use Zed's format command (default: `Cmd+Shift+I` on macOS, `Ctrl+Shift+I` on Linux/Windows)
- **Format on Save**: Enable in Zed settings for automatic formatting
- **Format Selection**: Select text and use the format command

### Keyboard Shortcuts
- `Cmd/Ctrl + Shift + I`: Format document
- `Cmd/Ctrl + /`: Toggle line comment
- `Cmd/Ctrl + Shift + /`: Toggle block comment

## Example Files

### Basic Template
```mustache_json5
{
  // Configuration for the application
  name: "My App",
  version: 1.0,
  
  // Dynamic user data
  greeting: "Hello {{user.name}}!",
  
  // Conditional sections
  features: [
    {{#each features}}
    {
      id: {{@index}},
      name: "{{name}}",
      {{#if enabled}}
      enabled: true,
      {{/if}}
    }{{#unless @last}},{{/unless}}
    {{/each}}
  ],
  
  // Unescaped content
  content: {{{html.body}}}
}
```

### Advanced Template
```mustache_json5
{
  // Complex nested structure with templates
  navigation: [
    {{#each nav_items}}
    {
      label: "{{title}}",
      url: "{{url}}",
      {{#if children}}
      children: [
        {{#each children}}
        {
          label: "{{title}}",
          url: "{{url}}"
        }{{#unless @last}},{{/unless}}
        {{/each}}
      ],
      {{/if}}
      active: {{#if @first}}true{{else}}false{{/if}}
    }{{#unless @last}},{{/unless}}
    {{/each}}
  ],
  
  // Helper functions
  formatted_date: "{{formatDate created_at 'YYYY-MM-DD'}}",
  truncated_text: "{{truncate description 100}}",
  
  // Partials
  header: {{> components/header}},
  footer: {{> components/footer}}
}
```

## Grammar Features

This extension uses a custom Tree-sitter grammar that properly parses the combination of JSON5 and Mustache template syntax:

### JSON5 Support
- Unquoted object keys
- Single and double quoted strings
- Trailing commas in objects and arrays
- Line comments (`//`) and block comments (`/* */`)
- Hexadecimal and decimal number formats
- Infinity and NaN values

### Mustache Support
- **Interpolation**: `{{variable}}`
- **Sections**: `{{#section}}...{{/section}}`
- **Inverted sections**: `{{^section}}...{{/section}}`
- **Unescaped output**: `{{{variable}}}`
- **Partials**: `{{> partial}}`
- **Comments**: `{{! comment }}`
- **Helper expressions**: `{{helper param1 param2}}`
- **Block helpers**: `{{#helper}}...{{/helper}}`

## Troubleshooting

### Formatter Not Found
If you see "command not found" errors:
1. Verify the formatter is installed: `mustache-json5-fmt --version`
2. Check your PATH includes the installation directory
3. Restart Zed after installing the formatter

### Syntax Highlighting Issues
1. Ensure the file has the correct extension (`.mustache_json5` or `.mjson5`)
2. Try reloading the language server
3. Check the grammar files are properly installed

### Formatting Issues
1. Verify the file syntax is valid (check for parsing errors)
2. Test the formatter directly: `mustache-json5-fmt --stdin < your-file.mustache_json5`
3. Check formatter configuration options

## Extension Installation

### Method 1: Manual Installation
1. Download this extension directory
2. Copy to your Zed extensions folder:
   - **macOS**: `~/.config/zed/extensions/`
   - **Linux**: `~/.config/zed/extensions/`
   - **Windows**: `%APPDATA%\Zed\extensions\`
3. Restart Zed

### Method 2: From Source
```bash
git clone https://github.com/tree-sitter/tree-sitter-mustache_json5
cp -r tree-sitter-mustache_json5/editors/zed ~/.config/zed/extensions/mustache-json5
```

### Method 3: Package Installation
```bash
# If published to Zed extension registry
zed --install-extension mustache-json5
```

## Development

To modify or extend this extension:

1. **Grammar changes**: Edit the tree-sitter grammar in the parent repository
2. **Syntax highlighting**: Modify `queries/mustache_json5/highlights.scm`
3. **Indentation**: Update `queries/mustache_json5/indents.scm`
4. **Formatter integration**: Modify `extension.toml` formatter configuration

## Contributing

This extension is part of the larger tree-sitter-mustache_json5 project:

1. Fork the repository: https://github.com/tree-sitter/tree-sitter-mustache_json5
2. Make your changes in the `editors/zed/` directory
3. Test thoroughly with various Mustache JSON5 files
4. Submit a pull request with a clear description

## License

This extension is released under the MIT license, same as the parent tree-sitter-mustache_json5 project.