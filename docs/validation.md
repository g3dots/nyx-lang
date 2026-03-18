# Validation

## Decision

I chose to validate the front end in layers: keep the lexer covered by an end-to-end token
test, and add a parser suite that checks AST shape and operator precedence across the full
syntax Nyx currently parses.

## Why I Made It

For the lexer, broad token coverage is more useful early on than a large number of tiny
isolated cases. For the parser, broad structural coverage matters more: the tests need to
prove that precedence, grouped expressions, functions, calls, and conditionals all produce
the intended tree. A parser test suite that exercises those combinations acts like a compact
specification for the front end and makes regressions easy to spot.

## What This Means Right Now

- The lexer has a concrete correctness target instead of only manual REPL checking.
- The parser has a concrete correctness target for AST structure, precedence, and syntax
  coverage.
- `cargo test` now validates both tokenization and parsing before evaluator work begins.
