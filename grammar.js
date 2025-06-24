/**
 * @file MustacheJson5 grammar for tree-sitter
 * @author nuiz
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "mustache_json5",

  extras: ($) => [$.comment, /\s/],

  word: ($) => $.identifier,

  conflicts: ($) => [[$.json5_document, $._template_content]],

  rules: {
    source_file: ($) => $.document,

    document: ($) => choice($.json5_document, $.template_document),

    // Pure JSON5 documents (no mustache)
    json5_document: ($) => $._json5_value,

    // Template documents (contains mustache)
    template_document: ($) => repeat1($._template_content),

    _template_content: ($) =>
      choice(
        $.mustache_comment,
        $.mustache_section,
        $.mustache_inverted_section,
        $.mustache_interpolation,
        $.mustache_unescaped,
        $.mustache_partial,
        $._json5_value,
        $.text,
      ),

    // JSON5 Comments
    comment: (_) =>
      token(
        choice(seq("//", /[^\n]*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),
      ),

    // JSON5 Values
    _json5_value: ($) =>
      choice($.object, $.array, $.number, $.string, $.null, $.true, $.false),

    // JSON5 Object
    object: ($) =>
      seq(
        "{",
        optional(
          seq($._object_item, repeat(seq(",", $._object_item)), optional(",")),
        ),
        "}",
      ),

    _object_item: ($) =>
      choice(
        $.member,
        $.mustache_section,
        $.mustache_inverted_section,
        $.mustache_comment,
        $.text,
      ),

    member: ($) => seq(field("name", $.name), ":", field("value", $._value)),

    name: ($) =>
      choice(
        $.string,
        $.identifier,
        $.mustache_interpolation,
        $.mustache_unescaped,
      ),

    identifier: (_) => /[a-zA-Z_$][a-zA-Z0-9_$]*/,

    // JSON5 Array
    array: ($) =>
      seq(
        "[",
        optional(
          seq($._array_item, repeat(seq(",", $._array_item)), optional(",")),
        ),
        "]",
      ),

    _array_item: ($) =>
      choice(
        $._value,
        $.mustache_section,
        $.mustache_inverted_section,
        $.mustache_comment,
        $.text,
      ),

    _value: ($) =>
      choice(
        $._json5_value,
        $.mustache_interpolation,
        $.mustache_unescaped,
        $.mustache_partial,
      ),

    // JSON5 String
    string: (_) => {
      const double_quote = seq(
        '"',
        repeat(
          choice(
            seq("\\", choice('"', "\\", "b", "f", "n", "r", "t", "v", "/")),
            /[^"\\]/,
          ),
        ),
        '"',
      );
      const single_quote = seq(
        "'",
        repeat(
          choice(
            seq("\\", choice("'", "\\", "b", "f", "n", "r", "t", "v", "/")),
            /[^'\\]/,
          ),
        ),
        "'",
      );
      return token(choice(double_quote, single_quote));
    },

    // JSON5 Number
    number: (_) => {
      const hex_digit = /[0-9a-fA-F]+/;
      const hex_int = seq("0", /[xX]/, hex_digit);
      const dec_digit = /[0-9]/;
      const exp_part = seq(/[eE]/, optional(/[+-]/), repeat1(dec_digit));
      const int_literal = choice("0", seq(/[1-9]/, repeat(dec_digit)));
      const dec_literal = choice(
        seq(int_literal, ".", repeat(dec_digit), optional(exp_part)),
        seq(".", repeat1(dec_digit), optional(exp_part)),
        seq(int_literal, optional(exp_part)),
      );
      return token(
        seq(optional(/[+-]/), choice(hex_int, dec_literal, "Infinity", "NaN")),
      );
    },

    // JSON5 Literals
    null: (_) => "null",
    true: (_) => "true",
    false: (_) => "false",

    // Text content (everything that's not mustache or JSON5)
    text: (_) => token(prec(-1, /[^{}\[\]",:\s]+/)),

    // Mustache Elements
    mustache_comment: ($) => seq("{{", "!", $.comment_content, "}}"),

    comment_content: (_) => token(/[^}]+/),

    mustache_interpolation: ($) => seq("{{", $._expression, "}}"),

    mustache_unescaped: ($) =>
      choice(
        seq("{{", "{", $._expression, "}", "}}"),
        seq("{{", "&", $._expression, "}}"),
      ),

    mustache_partial: ($) => seq("{{", ">", $.partial_name, "}}"),

    partial_name: (_) => token(/[a-zA-Z_][a-zA-Z0-9_.-]*/),

    mustache_section: ($) =>
      seq(
        $.mustache_section_begin,
        repeat($._template_content),
        $.mustache_section_end,
      ),

    mustache_section_begin: ($) =>
      seq(
        "{{",
        "#",
        $.tag_name,
        optional($._expression),
        optional($.section_parameters),
        "}}",
      ),

    mustache_section_end: ($) => seq("{{", "/", $.tag_name, "}}"),

    mustache_inverted_section: ($) =>
      seq(
        $.mustache_inverted_section_begin,
        repeat($._template_content),
        $.mustache_inverted_section_end,
      ),

    mustache_inverted_section_begin: ($) =>
      seq(
        "{{",
        "^",
        $.tag_name,
        optional($._expression),
        optional($.section_parameters),
        "}}",
      ),

    mustache_inverted_section_end: ($) => seq("{{", "/", $.tag_name, "}}"),

    // Section parameters for helpers like {{#each items as |item index|}}
    section_parameters: ($) =>
      seq("as", "|", $.parameter, repeat($.parameter), "|"),

    parameter: (_) => token(/[a-zA-Z_][a-zA-Z0-9_.]*/),

    tag_name: (_) => token(/[a-zA-Z_][a-zA-Z0-9_.]*/),

    // Mustache Expressions
    _expression: ($) =>
      choice($.path_expression, $.identifier_expression, $.dot_expression),

    identifier_expression: (_) => token(/[a-zA-Z_][a-zA-Z0-9_]*/),

    path_expression: ($) =>
      seq($.identifier_expression, repeat1(seq(".", $.identifier_expression))),

    dot_expression: (_) => ".",
  },
});
