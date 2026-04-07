# Evaluator Design

## Decision

I implemented the evaluator as a tree-walking interpreter that traverses the AST directly and
produces values through an object system built on a Rust enum.

## Why I Made It

A tree-walking evaluator is the simplest architecture that turns parsed programs into running
behavior. It avoids the complexity of compiling to bytecode or building an intermediate
representation. The evaluator walks each AST node, evaluates its children, and returns an
`Object` value. This keeps the evaluation logic close to the language structure and makes it
straightforward to add new expression types.

The object system uses an `Object` enum rather than trait objects. This mirrors the same design
choice made in the parser's AST and keeps pattern matching direct. Each variant carries only the
data it needs: integers hold an `i64`, functions hold their parameter list, body, and captured
environment.

Error propagation follows a value-based approach: errors are represented as `Object::Error`
values that short-circuit evaluation as they bubble up. This avoids a separate error channel and
keeps the control flow visible in the evaluator code.

## What This Means Right Now

- The REPL evaluates expressions and prints their results instead of displaying the raw AST.
- Variables, functions, closures, conditionals, loops, strings, arrays, and hashes all evaluate
  to concrete values.
- Errors produce readable messages that identify the problem (type mismatch, unknown operator,
  missing identifier) without crashing the REPL session.
- The environment persists across REPL lines, so bindings carry over between inputs.
