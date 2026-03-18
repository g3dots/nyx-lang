# Language Scope

## Decision

I chose to start with a small but coherent core language surface and make sure it is fully
parsed before expanding the language further. The current syntax covers identifiers,
integers, booleans, prefix and infix operators, grouped expressions, `let` and `return`
statements, `if` / `else`, function literals, and call expressions.

## Why I Made It

This scope is large enough to model the basic pieces of a programming language without
expanding into features that would slow down the early implementation. It establishes how
values, expressions, control flow, and function application should look before evaluator work
adds runtime meaning.

## What This Means Right Now

- The project parses a recognizable language shape instead of only tokenizing one.
- The supported syntax is enough to describe variable bindings, function declarations,
  conditionals, and precedence-sensitive expressions.
- Future evaluator work can build on an AST shape that is already exercised by parser tests.
