; Increase indent for JSON5 structures
(object "{" @indent.begin)
(object "}" @indent.end)

(array "[" @indent.begin)
(array "]" @indent.end)

; Increase indent for Mustache sections
(mustache_section
  (mustache_section_begin) @indent.begin)
(mustache_section
  (mustache_section_end) @indent.end)

(mustache_inverted_section
  (mustache_inverted_section_begin) @indent.begin)
(mustache_inverted_section
  (mustache_inverted_section_end) @indent.end)

; Align object members
(object
  (member) @indent.align)

; Align array elements
(array
  (_) @indent.align)

; Don't indent mustache delimiters themselves
(mustache_section_begin) @indent.zero
(mustache_section_end) @indent.zero
(mustache_inverted_section_begin) @indent.zero
(mustache_inverted_section_end) @indent.zero

; Template content should be indented relative to surrounding structure
(template_document
  (_) @indent.auto)
