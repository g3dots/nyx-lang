# Rust Code Decisions

## Decision

I chose a few Rust-specific implementation patterns to keep the interpreter front end simple and explicit:

- I store lexer input as `Vec<u8>` and scan it one byte at a time.
- I model token kinds as an enum and keep each produced token as a small struct containing the token type and literal text.
- I represent the parsed AST with owned enums and structs instead of trait objects.
- I keep the Pratt parser's prefix and infix handlers in explicit registration tables keyed by token type.
- I use standard library I/O directly in the REPL, including explicit flushing before reading input.
- I use a small constructor API for tokens so call sites can pass either string slices or owned strings without extra noise.

## Why I Made It

Scanning bytes is a straightforward fit for the ASCII-oriented syntax the project currently
supports, and it keeps character movement predictable. Using an enum for token kinds makes
the valid token set explicit in the type system and works well with pattern matching inside the
lexer. An enum-based AST is a better fit for Rust than storing heterogeneous nodes behind
trait objects, because parser code and tests need to inspect concrete variants constantly.
Using explicit handler registration tables keeps the Pratt parser structure visible in project
code instead of hiding it behind a larger abstraction. Direct `std::io` usage keeps the REPL
transparent and avoids introducing extra abstractions before they are needed.
Accepting `impl Into<String>` in `Token::new` makes token creation ergonomic while still
ending with a consistent owned representation.

## What This Means Right Now

- The lexer logic is easy to follow because position tracking and lookahead are handled with plain indices and byte comparisons.
- Token handling is strongly typed, which makes tests and lexer branches easier to read.
- Parser code works with concrete AST variants directly, which keeps precedence handling and
  assertions straightforward.
- Extending expression parsing means adding an AST variant plus a registered parse function,
  not introducing another layer of runtime polymorphism.
- The REPL behavior is controlled directly in project code instead of being hidden behind a library wrapper.
- The current implementation is optimized for clarity and a small language surface, not for advanced Unicode handling or richer terminal behavior.
