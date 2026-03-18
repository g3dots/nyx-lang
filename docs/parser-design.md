# Parser Design

## Decision

I implemented the parser as a Pratt parser with an owned Rust AST made of enums and
structs instead of trait objects.

## Why I Made It

The parser is built around Pratt parsing, where each token type can register a prefix parser,
an infix parser, or both. That structure is easy to extend and it maps cleanly to the language
features Nyx currently supports: literals, prefix operators, infix operators, grouped
expressions, conditionals, function literals, and function calls.

In Rust, representing statements and expressions with trait objects would push a lot of parser
and test code through downcasts and indirection. Using enums makes the valid AST surface
explicit and keeps matching on node kinds direct.

I also chose a parser error model that collects readable parse errors instead of aborting on
the first one, which makes the REPL more useful while the language is still growing.

## What This Means Right Now

- The parser owns a `Lexer`, maintains `cur_token` and `peek_token`, and uses explicit
  precedence levels to resolve infix binding.
- AST string rendering is deterministic, which allows the precedence tests and the REPL to
  use the same representation.
- The REPL now shows parsed output or parser errors, which makes the parser milestone
  visible without waiting for the evaluator.
- Adding the next parser feature will mainly require a new AST node plus one registered parse
  function.
