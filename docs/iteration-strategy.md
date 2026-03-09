# Iteration Strategy

## Decision

I chose to build the project in clear stages, starting with a working lexer and a simple REPL before moving on to the parser and evaluator.

## Why I Made It

The lexer gives me an early milestone that is concrete and easy to verify. A REPL makes that work immediately visible because I can type input and see how the language front end responds. Printing tokens at this stage is a practical way to check progress while later stages of the interpreter are still unfinished.

## What This Means Right Now

- Running the project starts an interactive loop instead of executing full programs.
- The current feedback loop is based on token output, not evaluated results.
- The README correctly presents the parser and evaluator as work still in progress.
