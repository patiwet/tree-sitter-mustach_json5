; JSON5 brackets
("[" @open "]" @close)
("{" @open "}" @close)
("(" @open ")" @close)

; JSON5 string quotes
("\"" @open "\"" @close)
("'" @open "'" @close)

; Mustache template brackets
("{{" @open "}}" @close)
("{{{" @open "}}}" @close)
