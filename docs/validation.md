# Validation

## Decision

I validate the interpreter in layers: the lexer has an end-to-end token test, the parser has a
structural test suite covering AST shape and operator precedence, and the evaluator has a
comprehensive test suite covering all expression types, data structures, control flow, closures,
built-in functions, and error handling.

## Why I Made It

Each layer has different failure modes. Lexer tests catch tokenization regressions. Parser tests
verify that precedence, grouping, and new syntax forms produce the intended tree. Evaluator tests
verify that the full pipeline — lex, parse, evaluate — produces correct values. Together they
form a compact specification for the language.

The evaluator tests cover integer arithmetic, boolean logic, prefix and infix operators,
if/else, return statements, error messages, let bindings, function application, closures,
strings, string concatenation, built-in functions, arrays, array indexing, hashes, hash indexing,
assignment, while loops, for loops, recursive functions, and higher-order functions.

## What This Means Right Now

- `cargo test` validates tokenization, parsing, and evaluation.
- New language features can be added with confidence that existing behavior is preserved.
- The evaluator test suite acts as both a regression guard and a usage reference for the
  language.
