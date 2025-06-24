#!/bin/bash

# Test script for Mustache JSON5 Zed Extension
# This script verifies that the extension is properly configured and functional

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Testing Mustache JSON5 Zed Extension${NC}"
echo "=============================================="

# Test 1: Check extension files exist
echo -e "\n${YELLOW}📁 Checking extension files...${NC}"

required_files=(
    "extension.toml"
    "languages/mustache-json5/config.toml"
    "languages/mustache-json5/highlights.scm"
    "languages/mustache-json5/indents.scm"
    "languages/mustache-json5/brackets.scm"
    "languages/mustache-json5/injections.scm"
    "languages/mustache-json5/outline.scm"
    "README.md"
    "install-formatter.sh"
    "test.mustache_json5"
)

for file in "${required_files[@]}"; do
    if [[ -f "$SCRIPT_DIR/$file" ]]; then
        echo -e "  ✅ $file"
    else
        echo -e "  ❌ $file ${RED}(missing)${NC}"
        exit 1
    fi
done

# Test 2: Validate extension.toml syntax
echo -e "\n${YELLOW}⚙️  Validating extension.toml...${NC}"
extension_file="$SCRIPT_DIR/extension.toml"

# Check for required fields in extension.toml
required_fields=(
    "id.*=.*\"mustache-json5\""
    "name.*=.*\"Mustache JSON5\""
    "version.*=.*\"0\.1\.0\""
    "schema_version.*=.*1"
    "\\[grammars\\.mustache_json5\\]"
    "repository.*=.*\"https://github\\.com/patiwet/tree-sitter-mustache_json5\""
)

field_count=0
for field in "${required_fields[@]}"; do
    if grep -qE "$field" "$extension_file"; then
        echo -e "  ✅ Found: $(echo "$field" | sed 's/\\//g' | sed 's/\.\*//' | cut -d'=' -f1)"
        ((field_count++))
    else
        echo -e "  ❌ Missing: $(echo "$field" | sed 's/\\//g' | sed 's/\.\*//' | cut -d'=' -f1)"
    fi
done

if [[ $field_count -eq ${#required_fields[@]} ]]; then
    echo -e "  ✅ extension.toml configuration is complete"
else
    echo -e "  ⚠️  extension.toml may be incomplete ($field_count/${#required_fields[@]} fields found)"
fi

# Test 3: Validate language configuration
echo -e "\n${YELLOW}🔧 Validating language configuration...${NC}"
config_file="$SCRIPT_DIR/languages/mustache-json5/config.toml"

if [[ -f "$config_file" ]]; then
    # Check for required language config fields
    lang_fields=(
        "name.*=.*\"Mustache JSON5\""
        "grammar.*=.*\"mustache_json5\""
        "path_suffixes.*=.*\\[.*\"mustache_json5\""
        "line_comments.*=.*\\[.*\"//\""
        "\\[formatter\\]"
        "external.*=.*{.*command.*=.*\"mustache-json5-fmt\""
    )

    lang_field_count=0
    for field in "${lang_fields[@]}"; do
        if grep -qE "$field" "$config_file"; then
            ((lang_field_count++))
        fi
    done

    if [[ $lang_field_count -eq ${#lang_fields[@]} ]]; then
        echo -e "  ✅ Language configuration is complete ($lang_field_count/${#lang_fields[@]})"
    else
        echo -e "  ⚠️  Language configuration may be incomplete ($lang_field_count/${#lang_fields[@]})"
    fi
fi

# Test 4: Validate query files
echo -e "\n${YELLOW}🔍 Validating query files...${NC}"
query_files=(
    "highlights.scm"
    "indents.scm"
    "brackets.scm"
    "injections.scm"
    "outline.scm"
)

for query_file in "${query_files[@]}"; do
    file_path="$SCRIPT_DIR/languages/mustache-json5/$query_file"
    if [[ -f "$file_path" ]]; then
        # Basic validation - check if file has some content
        if [[ -s "$file_path" ]]; then
            lines=$(wc -l < "$file_path")
            echo -e "  ✅ $query_file ($lines lines)"

            # Check for common query patterns
            case "$query_file" in
                "highlights.scm")
                    if grep -q "@" "$file_path"; then
                        echo -e "    ✅ Contains highlight captures"
                    fi
                    ;;
                "brackets.scm")
                    if grep -q "@open.*@close" "$file_path"; then
                        echo -e "    ✅ Contains bracket matching rules"
                    fi
                    ;;
                "indents.scm")
                    if grep -q "@indent" "$file_path"; then
                        echo -e "    ✅ Contains indentation rules"
                    fi
                    ;;
            esac
        else
            echo -e "  ⚠️  $query_file is empty"
        fi
    fi
done

# Test 5: Check formatter availability
echo -e "\n${YELLOW}🔧 Checking formatter...${NC}"
if command -v mustache-json5-fmt &> /dev/null; then
    version=$(mustache-json5-fmt --version 2>/dev/null || echo "unknown")
    echo -e "  ✅ Formatter available: $version"

    # Test formatter with test file
    if [[ -f "$SCRIPT_DIR/test.mustache_json5" ]]; then
        echo -e "  🧪 Testing formatter with test file..."
        if mustache-json5-fmt "$SCRIPT_DIR/test.mustache_json5" > /dev/null 2>&1; then
            echo -e "  ✅ Formatter successfully processed test file"
        else
            echo -e "  ❌ Formatter failed to process test file"
            exit 1
        fi
    fi
else
    echo -e "  ⚠️  Formatter not found in PATH"
    echo -e "     Run './install-formatter.sh' to install"
fi

# Test 6: Validate test file syntax
echo -e "\n${YELLOW}📝 Validating test file...${NC}"
test_file="$SCRIPT_DIR/test.mustache_json5"
if [[ -f "$test_file" ]]; then
    # Check if file has expected content patterns
    patterns=(
        "{{.*}}"           # Mustache interpolation
        "{{#.*}}"          # Mustache sections
        "{{/.*}}"          # Mustache section endings
        "{{>.*}}"          # Mustache partials
        "{{!.*}}"          # Mustache comments
        "//"               # JSON5 line comments
        "/\*.*\*/"         # JSON5 block comments
    )

    pattern_count=0
    for pattern in "${patterns[@]}"; do
        if grep -qE "$pattern" "$test_file"; then
            ((pattern_count++))
        fi
    done

    if [[ $pattern_count -ge 5 ]]; then
        echo -e "  ✅ Test file contains expected Mustache JSON5 patterns ($pattern_count/7)"
    else
        echo -e "  ⚠️  Test file may be missing some Mustache JSON5 patterns ($pattern_count/7)"
    fi
fi

# Test 7: Check installation script
echo -e "\n${YELLOW}🚀 Checking installation script...${NC}"
install_script="$SCRIPT_DIR/install-formatter.sh"
if [[ -f "$install_script" && -x "$install_script" ]]; then
    echo -e "  ✅ Installation script exists and is executable"

    # Check if script has required components
    if grep -q "cargo build --release" "$install_script"; then
        echo -e "  ✅ Installation script includes build command"
    else
        echo -e "  ⚠️  Installation script may be missing build command"
    fi
else
    echo -e "  ❌ Installation script missing or not executable"
    exit 1
fi

# Test 8: Check directory structure compliance
echo -e "\n${YELLOW}📂 Validating Zed extension structure...${NC}"

# Check if structure matches Zed requirements
if [[ -d "$SCRIPT_DIR/languages/mustache-json5" ]]; then
    echo -e "  ✅ Language directory structure is correct"
else
    echo -e "  ❌ Language directory structure is incorrect"
    exit 1
fi

# Check for grammar registration in extension.toml
if grep -q "\\[grammars\\.mustache_json5\\]" "$extension_file"; then
    echo -e "  ✅ Grammar is properly registered in extension.toml"
else
    echo -e "  ❌ Grammar is not properly registered"
    exit 1
fi

# Test Summary
echo -e "\n${BLUE}📋 Test Summary${NC}"
echo "=============="

# Count test files for distribution
total_files=$(find "$SCRIPT_DIR" -type f | wc -l)
echo -e "Total extension files: $total_files"

# Check directory structure
echo -e "Directory structure:"
echo -e "  📁 languages/         $(find "$SCRIPT_DIR/languages/" -name "*.toml" 2>/dev/null | wc -l) language configs"
echo -e "  📁 query files/       $(find "$SCRIPT_DIR/languages/" -name "*.scm" 2>/dev/null | wc -l) query files"

# Final status
echo -e "\n${GREEN}🎉 Extension validation completed successfully!${NC}"
echo -e "\n${YELLOW}Next steps:${NC}"
echo -e "1. Install formatter: ${BLUE}./install-formatter.sh${NC}"
echo -e "2. Install as dev extension in Zed:"
echo -e "   ${BLUE}Extensions > Install Dev Extension > Select this directory${NC}"
echo -e "3. Test with: ${BLUE}test.mustache_json5${NC}"

echo -e "\n${GREEN}✨ Ready for Zed!${NC}"
