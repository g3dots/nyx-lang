# Iteration Strategy

## Decision

I chose to build the project in clear stages: lexer first, then a parser milestone that makes
syntax structure visible in the REPL, and only after that the evaluator.

## Why I Made It

The lexer gives me an early milestone that is concrete and easy to verify. The parser is the
next milestone because it turns token streams into a real program structure and forces the
language front end to handle precedence, block structure, and higher-level syntax. Using the
REPL to print the parsed program keeps each stage immediately visible while evaluation is
still unfinished.

## What This Means Right Now

- Running the project starts an interactive loop instead of executing full programs.
- The current feedback loop is based on parsed output and parser errors, not evaluated
  results.
- The parser is treated as a complete milestone, while evaluation remains the next major
  phase.
