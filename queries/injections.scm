; Inject JSON5 highlighting into string literals that contain JSON5-like content
((string) @injection.content
  (#match? @injection.content "^[\"'][\\s]*[{\\[]")
  (#set! injection.language "json"))

; Inject JavaScript into mustache expressions that look like JS
((identifier_expression) @injection.content
  (#match? @injection.content "^(console|window|document|Math|JSON|Array|Object)")
  (#set! injection.language "javascript"))

; Inject CSS into string values that appear to be CSS
((string) @injection.content
  (#match? @injection.content "(color|background|font|margin|padding|border)\\s*:")
  (#set! injection.language "css"))

; Inject HTML into string values that contain HTML tags
((string) @injection.content
  (#match? @injection.content "<[a-zA-Z][^>]*>")
  (#set! injection.language "html"))

; Inject SQL into strings that start with SQL keywords
((string) @injection.content
  (#match? @injection.content "(?i)^[\"']\\s*(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER)")
  (#set! injection.language "sql"))

; Inject regex into strings that look like regular expressions
((string) @injection.content
  (#match? @injection.content "^[\"']/.*?/[gimuy]*[\"']$")
  (#set! injection.language "regex"))

; Inject markdown into comment content
((comment_content) @injection.content
  (#match? @injection.content "(#|\\*|-|\\d+\\.|>)")
  (#set! injection.language "markdown"))

; Inject template languages into mustache comments for documentation
((comment_content) @injection.content
  (#match? @injection.content "@(param|return|example|deprecated)")
  (#set! injection.language "jsdoc"))
