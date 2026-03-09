# Language Scope

## Decision

I chose to start with a small but coherent core language surface. The current syntax covers identifiers, integers, assignment, arithmetic operators, comparison and equality operators, delimiters, function syntax, conditional keywords, boolean literals, and return statements.

## Why I Made It

This scope is large enough to model the basic pieces of a programming language without expanding into features that would slow down the early implementation. It lets me establish how values, expressions, and control flow should look before I take on the harder parts of parsing and evaluation.

## What This Means Right Now

- The project already defines a recognizable language shape instead of a random list of tokens.
- The supported tokens are enough to describe variable bindings, function declarations, conditionals, and simple expressions.
- Future parser and evaluator work can build on a syntax surface that is already outlined and tested.
