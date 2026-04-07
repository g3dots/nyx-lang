# Loops and Assignment

## Decision

I added `while` loops, C-style `for` loops, and assignment expressions to the language.

## Why I Made It

Loops require the ability to mutate state across iterations. The core language only has `let`
bindings, which always create a new binding in the current scope. Without mutation, a loop body
like `let x = x + 1` would shadow the outer `x` with a new local binding each iteration, never
changing the original. The loop condition would read the unchanged outer `x` and either run
forever or not at all.

Assignment expressions (`x = expr`) solve this by walking the environment chain to find and
update an existing binding. This is the minimal addition needed to make loops useful. Assignment
is parsed as an infix operator with the lowest precedence and is special-cased in the evaluator
to update the environment rather than performing arithmetic.

`while` follows the same structure as `if` — a parenthesized condition and a block body — but
repeats until the condition is falsy. `for` adds an init statement, condition, and update
expression in a C-style `for (init; cond; update) { body }` syntax. The init position accepts a
full statement so `let` bindings work there.

## What This Means Right Now

- `while (condition) { body }` loops until condition is falsy.
- `for (let i = 0; i < n; i = i + 1) { body }` provides bounded iteration.
- `x = expr` mutates an existing binding anywhere in the scope chain.
- Loops propagate `return` values and errors correctly, so `return` inside a loop exits
  the enclosing function.
