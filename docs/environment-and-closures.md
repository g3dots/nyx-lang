# Environment and Closures

## Decision

I implemented variable scoping using a linked chain of environment frames, where each frame
holds a `HashMap<String, Object>` and an optional reference to an outer frame. Environments are
wrapped in `Rc<RefCell<Environment>>` to support shared ownership and interior mutability.

## Why I Made It

Functions in Nyx are first-class values that can be returned, passed as arguments, and used as
closures. A closure needs to capture the environment it was defined in so it can access variables
from its enclosing scope even after that scope has exited. The linked environment chain handles
this naturally: when a function is created, it stores a reference to its defining environment.
When called, a new enclosed frame is pushed on top.

The `Rc<RefCell<>>` wrapper is the standard Rust approach for tree-walking interpreters. `Rc`
gives shared ownership across function values that close over the same scope. `RefCell` gives
interior mutability so variables can be set during evaluation without requiring exclusive
ownership of the entire chain.

The environment distinguishes `set` from `update`: `set` always writes to the current frame
(used by `let` bindings), while `update` walks the chain to find an existing binding and mutates
it in place. This distinction is what makes assignment expressions work correctly inside loops
and nested scopes.

## What This Means Right Now

- `let` creates a new binding in the current scope.
- Assignment (`x = expr`) mutates the nearest existing binding in the scope chain.
- Functions capture their environment and work as closures.
- Each function call gets its own scope frame with parameters bound as local variables.
