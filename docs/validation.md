# Validation

## Decision

I chose to validate the lexer with a representative end-to-end test that checks both token type and literal value across a realistic multi-line input sample.

## Why I Made It

For a front-end component like a lexer, broad token coverage is more useful early on than a large number of tiny isolated cases. A single realistic input string exercises keywords, operators, delimiters, literals, and control-flow syntax together. That makes the test act like a compact specification for the language surface I have implemented so far.

## What This Means Right Now

- The lexer has a concrete correctness target instead of only manual REPL checking.
- Changes to tokenization can be verified against expected output quickly.
- The current tests are strongest around lexical behavior, which matches the current stage of the project.
