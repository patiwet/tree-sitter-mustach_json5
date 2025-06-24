package tree_sitter_mustache_json5_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_mustache_json5 "github.com/tree-sitter/tree-sitter-mustache_json5/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_mustache_json5.Language())
	if language == nil {
		t.Errorf("Error loading MustacheJson5 grammar")
	}
}
