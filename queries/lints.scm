; Linting rules for common mustache_json5 issues

; Error: Unclosed mustache sections
(mustache_section_begin
  (tag_name) @error.unclosed.section
  (#not-has-ancestor? @error.unclosed.section mustache_section))

; Error: Mismatched section tags
(mustache_section
  (mustache_section_begin (tag_name) @start.tag)
  (mustache_section_end (tag_name) @end.tag)
  (#not-eq? @start.tag @end.tag)) @error.mismatched.section

(mustache_inverted_section
  (mustache_inverted_section_begin (tag_name) @start.tag)
  (mustache_inverted_section_end (tag_name) @end.tag)
  (#not-eq? @start.tag @end.tag)) @error.mismatched.inverted

; Warning: Empty mustache expressions
((mustache_interpolation) @warning.empty.expression
  (#match? @warning.empty.expression "^\\{\\{\\s*\\}\\}$"))

; Warning: Deprecated mustache syntax patterns
(mustache_interpolation
  (identifier_expression) @warning.deprecated
  (#match? @warning.deprecated "^(\\$|@|_)"))

; Error: Invalid JSON5 trailing commas in wrong places
((object
  ","
  "}") @error.trailing.comma)

((array
  ","
  "]") @error.trailing.comma)

; Warning: Potentially unsafe unescaped output
(mustache_unescaped
  (identifier_expression) @warning.unsafe.unescaped
  (#match? @warning.unsafe.unescaped "(html|script|style|user|input)"))

; Error: Invalid mustache section parameters syntax
((section_parameters) @error.invalid.params
  (#not-match? @error.invalid.params "^\\s*as\\s*\\|.*\\|\\s*$"))

; Warning: Unused section parameters
((section_parameters
  (parameter) @warning.unused.param))

; Error: Invalid path expressions (starting with dot)
(path_expression
  (identifier_expression) @error.invalid.path
  (#match? @error.invalid.path "^\\."))

; Warning: Deep nesting (more than 3 levels)
(mustache_section
  (mustache_section
    (mustache_section
      (mustache_section) @warning.deep.nesting)))

; Error: Missing required closing delimiters
((ERROR) @error.unclosed.delimiter
  (#match? @error.unclosed.delimiter "\\{\\{[^}]*$"))

; Warning: Inconsistent quote styles in JSON5
((object
   (member
     (name (string) @single.quote)
     (#match? @single.quote "^'"))
   (member
     (name (string) @double.quote)
     (#match? @double.quote "^\""))
  ) @warning.inconsistent.quotes)

; Error: Invalid JSON5 identifiers
((identifier) @error.invalid.identifier
  (#match? @error.invalid.identifier "^\\d"))

; Warning: Mustache comments in production code
(mustache_comment
  (comment_content) @warning.debug.comment
  (#match? @warning.debug.comment "(TODO|FIXME|DEBUG|HACK|XXX)"))

; Error: Malformed mustache partials
(mustache_partial
  (partial_name) @error.invalid.partial
  (#match? @error.invalid.partial "[^a-zA-Z0-9_.-]"))

; Warning: Long mustache expressions (readability)
((mustache_interpolation) @warning.long.expression
  (#match? @warning.long.expression ".{50,}"))

; Error: Reserved mustache keywords used as variables
((identifier_expression) @error.reserved.keyword
  (#any-of? @error.reserved.keyword "if" "unless" "each" "with" "unless" "else"))

; Warning: Empty JSON5 objects or arrays
((object) @warning.empty.object)

((array) @warning.empty.array)
