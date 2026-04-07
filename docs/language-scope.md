# Language Scope

## Decision

I built the language in layers, starting with a core syntax of identifiers, integers, booleans,
operators, `let`, `return`, `if`/`else`, function literals, and call expressions. I then expanded
the surface to include strings, arrays, hashes, index expressions, `while` loops, `for` loops,
assignment expressions, and built-in functions (`len`, `puts`, `first`, `last`, `rest`, `push`).

## Why I Made It

The initial core was large enough to model expressions, control flow, and function application
without overreaching. Once the parser and evaluator were stable for that core, each additional
feature could be added independently: a new token, AST node, parse function, and eval case. This
incremental approach kept each expansion small and testable.

Strings, arrays, and hashes make the language practical for non-trivial programs. Loops and
assignment make state mutation possible. Built-in functions provide the minimal standard library
needed to work with the data types the language supports.

## What This Means Right Now

- The language supports six data types: integers, booleans, strings, arrays, hashes, and
  functions (including closures).
- Control flow covers `if`/`else`, `while`, and `for`.
- The built-in function set is small and focused on collection operations.
- The evaluator fully executes programs, so the REPL shows evaluated results rather than
  parsed AST.
