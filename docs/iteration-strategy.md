# Iteration Strategy

## Decision

I built the project in clear stages: lexer first, then parser, then evaluator. Each stage was
treated as a complete milestone with its own test coverage and visible feedback in the REPL.

## Why I Made It

The lexer gives an early milestone that is concrete and easy to verify. The parser turns token
streams into a real program structure and forces the front end to handle precedence, block
structure, and higher-level syntax. The evaluator turns the AST into running behavior. Building
in this order means each stage can be validated independently before the next one depends on it.

## What This Means Right Now

- The REPL evaluates input and prints results rather than showing parsed output.
- All three stages — lexer, parser, evaluator — have their own test suites.
- The language surface was expanded alongside the evaluator to include strings, arrays, hashes,
  loops, assignment, and built-in functions.
