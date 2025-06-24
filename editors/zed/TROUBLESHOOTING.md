# Zed Extension Troubleshooting Guide

This guide helps resolve common issues when installing and using the Mustache JSON5 extension for Zed.

## 🚨 Common Installation Issues

### Issue: "failed to compile grammar 'mustache_json5'"

This error occurs when Zed cannot compile the tree-sitter grammar. Here are the solutions:

#### Solution 1: Use the Self-Contained Extension
Make sure you're using the latest packaged extension that includes all grammar files:

```bash
# Download the latest extension package
cd ~/.config/zed/extensions/
rm -rf mustache-json5  # Remove old version if exists

# Extract the new package
tar -xzf mustache-json5-zed-extension-0.1.0.tar.gz
mv mustache-json5-zed-extension mustache-json5
```

#### Solution 2: Verify Grammar Files
Check that the grammar directory contains all necessary files:

```bash
ls ~/.config/zed/extensions/mustache-json5/grammars/mustache_json5/
# Should contain: grammar.js, src/, queries/, package.json, Cargo.toml
```

#### Solution 3: Manual Grammar Compilation
If the grammar still won't compile, try manual compilation:

```bash
cd ~/.config/zed/extensions/mustache-json5/grammars/mustache_json5/
tree-sitter generate
tree-sitter build --wasm
```

### Issue: "Command not found: mustache-json5-fmt"

The formatter binary is not in your PATH or not installed.

#### Solution 1: Install the Formatter
```bash
# From the extension directory
cd ~/.config/zed/extensions/mustache-json5/
./install-formatter.sh
```

#### Solution 2: Manual Binary Installation
```bash
# Download and install the formatter binary
curl -L -o mustache-json5-fmt https://github.com/patiwet/tree-sitter-mustache_json5/releases/latest/download/mustache-json5-fmt-$(uname -s)-$(uname -m)
chmod +x mustache-json5-fmt
sudo mv mustache-json5-fmt /usr/local/bin/
```

#### Solution 3: Verify Installation
```bash
which mustache-json5-fmt
mustache-json5-fmt --version
```

### Issue: "Extension not loading in Zed"

#### Solution 1: Check Extension Directory
```bash
ls ~/.config/zed/extensions/mustache-json5/
# Should contain: extension.toml, grammars/, queries/, languages/
```

#### Solution 2: Verify Extension Configuration
Check that `~/.config/zed/extensions/mustache-json5/extension.toml` exists and is valid:

```toml
id = "mustache-json5"
name = "Mustache JSON5"
version = "0.1.0"
# ... rest of configuration
```

#### Solution 3: Restart Zed
After installing or updating the extension:
1. Quit Zed completely
2. Restart Zed
3. Open a `.mustache_json5` file to test

### Issue: "Syntax highlighting not working"

#### Solution 1: Check File Extension
Make sure your files use the correct extensions:
- `.mustache_json5` (preferred)
- `.mjson5` (alternative)

#### Solution 2: Verify Query Files
```bash
ls ~/.config/zed/extensions/mustache-json5/queries/
# Should contain: highlights.scm, indents.scm, etc.
```

#### Solution 3: Force Language Mode
In Zed:
1. Open Command Palette (Cmd+Shift+P)
2. Type "Select Language"
3. Choose "Mustache JSON5"

### Issue: "Formatting not working"

#### Solution 1: Test Formatter Directly
```bash
echo '{"test":"{{value}}"}' | mustache-json5-fmt --stdin
```

#### Solution 2: Check Zed Settings
Add to your Zed `settings.json`:
```json
{
  "languages": {
    "Mustache JSON5": {
      "formatter": {
        "external": {
          "command": "mustache-json5-fmt",
          "arguments": ["--stdin"]
        }
      }
    }
  }
}
```

#### Solution 3: Manual Format
Try manual formatting:
1. Right-click in editor
2. Select "Format Document"
3. Or use Cmd+Shift+I (Mac) / Ctrl+Shift+I (Linux/Windows)

## 🔧 Debug Commands

### Check Extension Status
```bash
# List installed extensions
ls ~/.config/zed/extensions/

# Check mustache-json5 extension files
find ~/.config/zed/extensions/mustache-json5/ -type f | head -10

# Verify formatter
which mustache-json5-fmt && mustache-json5-fmt --version
```

### Test Grammar Compilation
```bash
cd ~/.config/zed/extensions/mustache-json5/grammars/mustache_json5/
tree-sitter test 2>&1 | head -5
```

### Test File Recognition
Create a test file:
```bash
echo '{"name": "{{user.name}}", "id": {{user.id}}}' > test.mustache_json5
```

Open it in Zed and check if syntax highlighting appears.

## 🏗️ Complete Reinstallation

If nothing else works, try a complete clean reinstall:

```bash
# 1. Remove old extension
rm -rf ~/.config/zed/extensions/mustache-json5/

# 2. Remove formatter
sudo rm -f /usr/local/bin/mustache-json5-fmt

# 3. Download fresh extension package
curl -L -o mustache-json5-zed-extension.tar.gz \
  https://github.com/patiwet/tree-sitter-mustache_json5/releases/latest/download/mustache-json5-zed-extension-0.1.0.tar.gz

# 4. Install extension
mkdir -p ~/.config/zed/extensions/
cd ~/.config/zed/extensions/
tar -xzf ~/mustache-json5-zed-extension.tar.gz
mv mustache-json5-zed-extension mustache-json5

# 5. Install formatter
cd mustache-json5
./install-formatter.sh

# 6. Restart Zed
```

## 📝 System Requirements

- **Zed Editor**: Latest version recommended
- **Operating System**: macOS, Linux (Windows support varies)
- **Tree-sitter CLI**: For manual grammar compilation (optional)
- **Rust**: For building from source (optional)

## 🆘 Still Having Issues?

If you're still experiencing problems:

1. **Check Zed Logs**: Look for error messages in Zed's developer console
2. **File an Issue**: [GitHub Issues](https://github.com/patiwet/tree-sitter-mustache_json5/issues)
3. **Provide Details**:
   - Zed version (`Zed > About Zed`)
   - Operating system and version
   - Error messages (exact text)
   - Steps to reproduce
   - Contents of problematic files

## ✅ Verification Checklist

After installation, verify everything works:

- [ ] Extension appears in `~/.config/zed/extensions/mustache-json5/`
- [ ] Formatter binary works: `mustache-json5-fmt --version`
- [ ] Test file shows syntax highlighting: create `test.mustache_json5`
- [ ] Formatting works: Right-click → "Format Document"
- [ ] File extensions recognized: `.mustache_json5`, `.mjson5`

If all items are checked, your installation is complete! 🎉