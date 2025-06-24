; Outline structure for Mustache JSON5 files

; JSON5 object members (key-value pairs)
(object
  (member
    key: (string (string_content) @name)) @item)

(object
  (member
    key: (identifier) @name) @item)

; JSON5 array elements with indices
(array
  (value) @item
  (#set! name-prefix "[")
  (#set! name-suffix "]"))

; Mustache sections
(mustache_section
  (mustache_section_begin
    (tag_name) @name) @item
  (#set! kind "section"))

; Mustache inverted sections
(mustache_inverted_section
  (mustache_inverted_section_begin
    (tag_name) @name) @item
  (#set! kind "inverted-section"))

; Mustache partials
(mustache_partial
  (partial_name) @name @item
  (#set! kind "partial"))

; Mustache comments (for documentation structure)
(mustache_comment
  (comment_content) @name @item
  (#set! kind "comment")
  (#match? @name "(TODO|FIXME|NOTE|DOC)"))

; Top-level template blocks
(template_document) @context
