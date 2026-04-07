# nyx

A programming language interpreter written in Rust. Zero dependencies.

## Usage

```bash
cargo run
```

Starts a REPL. Type expressions at the `>>` prompt to see evaluated results.

```
>> 5 + 5
10
>> let greet = fn(name) { "hello " + name };
>> greet("world")
hello world
>> let xs = [1, 2, 3]; xs[1]
2
>> {"a": 1, "b": 2}["b"]
2
>> let x = 0; while (x < 3) { x = x + 1; }; x;
3
>> let sum = 0; for (let i = 0; i < 5; i = i + 1) { sum = sum + i; }; sum;
10
```

## Language Features

**Types:** integers, booleans, strings, arrays, hashes, functions, null

**Operators:** `+` `-` `*` `/` `<` `>` `==` `!=` `!` `-` (prefix)

**Bindings and assignment:**
```
let x = 5;
x = x + 1;
```

**Control flow:**
```
if (x > 0) { x } else { -x }
while (x < 10) { x = x + 1; }
for (let i = 0; i < 10; i = i + 1) { puts(i); }
```

**Functions and closures:**
```
let add = fn(a, b) { a + b };
let adder = fn(x) { fn(y) { x + y } };
adder(2)(3)
```

**Builtins:** `len`, `puts`, `first`, `last`, `rest`, `push`

## Tests

```bash
cargo test
```

## Architecture

Lexer → Parser (Pratt) → AST → Tree-walking evaluator

See [docs/](docs/) for design decisions.
