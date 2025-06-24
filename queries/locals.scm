; Variable definitions and references for Mustache templates

; Section parameters define local variables
(section_parameters
  (parameter) @local.definition.variable)

; Mustache interpolations reference variables
(mustache_interpolation
  (identifier_expression) @local.reference)

(mustache_unescaped
  (identifier_expression) @local.reference)

; Path expressions - first part is the base reference
(path_expression
  (identifier_expression) @local.reference)

; Section expressions reference variables
(mustache_section_begin
  (identifier_expression) @local.reference)

(mustache_inverted_section_begin
  (identifier_expression) @local.reference)

; Scope definitions
(mustache_section) @local.scope

(mustache_inverted_section) @local.scope

; Template document as global scope
(template_document) @local.scope

; JSON5 object property names are not variables
(name
  (identifier) @local.definition.property)

; Partial references
(mustache_partial
  (partial_name) @local.reference.partial)
