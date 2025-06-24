; JSON5 object folding
(object
  "{" @fold.start
  "}" @fold.end) @fold

; JSON5 array folding
(array
  "[" @fold.start
  "]" @fold.end) @fold

; Mustache section folding
(mustache_section
  (mustache_section_begin) @fold.start
  (mustache_section_end) @fold.end) @fold

; Mustache inverted section folding
(mustache_inverted_section
  (mustache_inverted_section_begin) @fold.start
  (mustache_inverted_section_end) @fold.end) @fold

; Multi-line JSON5 comments
(comment) @fold
  (#match? @fold "/\\*[\\s\\S]*?\\*/")

; Template document folding (for large template blocks)
(template_document) @fold
  (#has-parent? template_document)
