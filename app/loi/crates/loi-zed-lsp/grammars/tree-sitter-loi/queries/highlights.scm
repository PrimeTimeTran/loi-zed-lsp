; grammars/tree-sitter-loi/queries/highlights.scm
"let" @keyword
(let_statement
  name: (identifier) @variable)
(identifier) @variable
(comment) @comment