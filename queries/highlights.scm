; Mustache delimiters and operators
["{{" "}}"] @punctuation.bracket

; Mustache keywords and operators
["#" "^" "/" "!" ">" "&"] @operator

; Mustache section keywords
(tag_name) @keyword

; Mustache parameters and variables
(parameter) @variable.parameter
(identifier_expression) @variable
(path_expression (identifier_expression) @variable.member)

; Mustache comments
(mustache_comment) @comment
(comment_content) @comment

; JSON5 structural elements
["{" "}" "[" "]"] @punctuation.bracket
[":" ","] @punctuation.delimiter

; JSON5 literals
(string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin

; JSON5 object keys
(name (identifier) @property)
(name (string) @string)

; JSON5 comments
(comment) @comment

; Special Mustache syntax
"as" @keyword
["|" "|"] @punctuation.bracket

; Mustache partial names
(partial_name) @function

; Text content
(text) @none

; Error highlighting
(ERROR) @error
