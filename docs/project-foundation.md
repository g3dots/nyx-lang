# Project Foundation

## Decision

I chose to build Nyx as a programming language interpreter in Rust. I also kept the dependency list empty and organized the project so the executable entrypoint stays small while the language logic lives in reusable modules.

## Why I Made It

Rust is a good fit for an interpreter project because it gives me explicit control over data structures and runtime behavior. Keeping dependencies at zero keeps the implementation easy to inspect and makes the core ideas belong to the project itself instead of external crates. Splitting the entrypoint from the language modules also keeps the structure clean as the project grows.

## What This Means Right Now

- The codebase is small and direct.
- Core behavior is grouped into focused modules such as the lexer, token definitions, and REPL.
- The executable mainly starts the program and hands control to the interpreter code instead of containing the implementation itself.
