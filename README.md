# nyx

A programming language interpreter written in Rust.

## Status

Early development — lexer and parser complete, evaluator in progress.

## Usage

```bash
cargo run
```

Starts a REPL. Type expressions at the `>>` prompt to see the parsed AST string form. If the
input is syntactically invalid, the REPL prints parser errors instead.

## Roadmap

- [x] Parser / AST
- [ ] Evaluator
