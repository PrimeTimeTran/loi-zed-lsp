// grammars/tree-sitter-loi/grammar.js
module.exports = grammar({
  name: "loi",
  extras: $ => [
    /\s/,
    $.comment,
  ],
  rules: {
    source_file: $ =>
      repeat($._statement),
    _statement: $ =>
      choice(
        $.let_statement,
        $.identifier
      ),
    let_statement: $ =>
      seq(
        "let",
        field("name", $.identifier)
      ),
    identifier: $ =>
      /[a-zA-Z_][a-zA-Z0-9_]*/,
    comment: $ =>
      token(seq(
        "#",
        /.*/
      )),
  }
});