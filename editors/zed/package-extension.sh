#!/bin/bash

# Package script for Mustache JSON5 Zed Extension
# Creates a distributable package of the extension

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_NAME="mustache-json5-zed-extension"
VERSION="0.1.0"
DIST_DIR="$SCRIPT_DIR/dist"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📦 Packaging Mustache JSON5 Zed Extension${NC}"
echo "=============================================="

# Clean and create distribution directory
echo -e "\n${YELLOW}🧹 Preparing distribution directory...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Create package directory structure
PACKAGE_DIR="$DIST_DIR/$PACKAGE_NAME"
mkdir -p "$PACKAGE_DIR"

# Copy extension files
echo -e "\n${YELLOW}📁 Copying extension files...${NC}"
files_to_copy=(
    "extension.toml"
    "README.md"
    "package.json"
    "install-formatter.sh"
    "test.mustache_json5"
    "test-extension.sh"
)

for file in "${files_to_copy[@]}"; do
    if [[ -f "$SCRIPT_DIR/$file" ]]; then
        cp "$SCRIPT_DIR/$file" "$PACKAGE_DIR/"
        echo -e "  ✅ $file"
    else
        echo -e "  ❌ $file ${RED}(missing)${NC}"
        exit 1
    fi
done

# Copy directories
echo -e "\n${YELLOW}📂 Copying directories...${NC}"
directories_to_copy=(
    "languages"
    "grammars"
)

for dir in "${directories_to_copy[@]}"; do
    if [[ -d "$SCRIPT_DIR/$dir" ]]; then
        cp -r "$SCRIPT_DIR/$dir" "$PACKAGE_DIR/"
        file_count=$(find "$PACKAGE_DIR/$dir" -type f | wc -l)
        echo -e "  ✅ $dir/ ($file_count files)"
    else
        echo -e "  ❌ $dir/ ${RED}(missing)${NC}"
        exit 1
    fi
done

# Make scripts executable
echo -e "\n${YELLOW}🔧 Setting permissions...${NC}"
chmod +x "$PACKAGE_DIR/install-formatter.sh"
chmod +x "$PACKAGE_DIR/test-extension.sh"
echo -e "  ✅ Made scripts executable"

# Create installation instructions
echo -e "\n${YELLOW}📝 Creating installation instructions...${NC}"
cat > "$PACKAGE_DIR/INSTALL.md" << 'EOF'
# Installation Instructions

## Quick Install

1. **Install the formatter**:
   ```bash
   ./install-formatter.sh
   ```

2. **Install the extension**:
   ```bash
   # Copy to Zed extensions directory
   cp -r . ~/.config/zed/extensions/mustache-json5/
   ```

3. **Restart Zed** and open a `.mustache_json5` file

## Manual Install Steps

### 1. Install Formatter

#### Option A: Automatic (Recommended)
```bash
./install-formatter.sh
```

#### Option B: Manual Build
```bash
# You'll need the full tree-sitter-mustache_json5 repository
git clone https://github.com/tree-sitter/tree-sitter-mustache_json5
cd tree-sitter-mustache_json5/formatter
cargo build --release
cp target/release/mustache-json5-fmt ~/.local/bin/
```

### 2. Install Extension

#### Linux/macOS
```bash
mkdir -p ~/.config/zed/extensions/
cp -r . ~/.config/zed/extensions/mustache-json5/
```

#### Windows
```powershell
mkdir -p $env:APPDATA\Zed\extensions\
cp -r . $env:APPDATA\Zed\extensions\mustache-json5\
```

### 3. Verify Installation

1. Run the test script:
   ```bash
   ./test-extension.sh
   ```

2. Open `test.mustache_json5` in Zed

3. Test formatting with `Cmd+Shift+I` (macOS) or `Ctrl+Shift+I` (Linux/Windows)

## Troubleshooting

- **Formatter not found**: Ensure `mustache-json5-fmt` is in your PATH
- **No syntax highlighting**: Check file extension is `.mustache_json5` or `.mjson5`
- **Extension not loading**: Restart Zed after installation

## Support

For issues, visit: https://github.com/tree-sitter/tree-sitter-mustache_json5/issues
EOF

echo -e "  ✅ Created INSTALL.md"

# Create version info
echo -e "\n${YELLOW}📋 Creating version info...${NC}"
cat > "$PACKAGE_DIR/VERSION" << EOF
Package: $PACKAGE_NAME
Version: $VERSION
Built: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Platform: $(uname -s) $(uname -m)

Files included:
- Extension configuration (extension.toml)
- Language definition (languages/mustache-json5/config.toml)
- Tree-sitter grammar (grammars/)
- Syntax highlighting queries (languages/mustache-json5/*.scm)
- Formatter installation script (install-formatter.sh)
- Test files and documentation

Requirements:
- Zed editor >= 0.120.0
- Rust/Cargo (for formatter installation)
- mustache-json5-fmt formatter
EOF

echo -e "  ✅ Created VERSION"

# Create different package formats
echo -e "\n${YELLOW}📦 Creating package formats...${NC}"

# 1. Tar.gz package
cd "$DIST_DIR"
tar -czf "$PACKAGE_NAME-$VERSION.tar.gz" "$PACKAGE_NAME"
echo -e "  ✅ Created $PACKAGE_NAME-$VERSION.tar.gz"

# 2. Zip package
if command -v zip &> /dev/null; then
    zip -r "$PACKAGE_NAME-$VERSION.zip" "$PACKAGE_NAME" > /dev/null
    echo -e "  ✅ Created $PACKAGE_NAME-$VERSION.zip"
else
    echo -e "  ⚠️  zip command not found, skipping .zip package"
fi

# 3. Directory package (for direct copying)
echo -e "  ✅ Created directory package: $PACKAGE_NAME/"

# Generate checksums
echo -e "\n${YELLOW}🔒 Generating checksums...${NC}"
if command -v sha256sum &> /dev/null; then
    sha256sum "$PACKAGE_NAME-$VERSION.tar.gz" > "$PACKAGE_NAME-$VERSION.tar.gz.sha256"
    echo -e "  ✅ SHA256 checksum created"
elif command -v shasum &> /dev/null; then
    shasum -a 256 "$PACKAGE_NAME-$VERSION.tar.gz" > "$PACKAGE_NAME-$VERSION.tar.gz.sha256"
    echo -e "  ✅ SHA256 checksum created"
else
    echo -e "  ⚠️  No SHA256 tool found, skipping checksum"
fi

# Package summary
echo -e "\n${BLUE}📊 Package Summary${NC}"
echo "=================="

cd "$DIST_DIR"
echo -e "Distribution directory: ${BLUE}$DIST_DIR${NC}"
echo -e "Package formats created:"

for file in "$PACKAGE_NAME-$VERSION".*; do
    if [[ -f "$file" ]]; then
        size=$(du -h "$file" | cut -f1)
        echo -e "  📦 $file (${size})"
    fi
done

if [[ -d "$PACKAGE_NAME" ]]; then
    file_count=$(find "$PACKAGE_NAME" -type f | wc -l)
    echo -e "  📁 $PACKAGE_NAME/ directory ($file_count files)"
fi

echo -e "\n${GREEN}🎉 Packaging completed successfully!${NC}"

echo -e "\n${YELLOW}📋 Next Steps:${NC}"
echo -e "1. Test the package:"
echo -e "   ${BLUE}cd $DIST_DIR/$PACKAGE_NAME && ./test-extension.sh${NC}"
echo -e ""
echo -e "2. Distribute the package:"
echo -e "   - Upload ${BLUE}$PACKAGE_NAME-$VERSION.tar.gz${NC} to releases"
echo -e "   - Share installation instructions from ${BLUE}INSTALL.md${NC}"
echo -e ""
echo -e "3. Install locally:"
echo -e "   ${BLUE}cd $DIST_DIR/$PACKAGE_NAME && ./install-formatter.sh${NC}"
echo -e "   ${BLUE}cp -r $DIST_DIR/$PACKAGE_NAME ~/.config/zed/extensions/mustache-json5/${NC}"

echo -e "\n${GREEN}✨ Ready for distribution!${NC}"
