# Control Flow

Rune supports your typical forms of control flow.

## `return` Keyword

The `return` keyword allows for returning from the current function.
If specified without an argument, the function will return a unit `()`.

The last statement in a function is known as an *implicit return*, and will be
what the function returns by default unless a `return` is specified.

```rust,noplaypen
{{#include ../../scripts/book/4_3/numbers_game.rn}}
```

```text
$> cargo run -- scripts/book/4_3/numbers_game.rn
less than one
something else
== () (3.8608ms)
```

## `if` Expressions

If expressions allow you to provide a condition with one or more code branches.
If the condition is `true`, the provided block of code will run.

```rust,noplaypen
{{#include ../../scripts/book/4_3/conditional.rn}}
```

```text
$> cargo run -- scripts/book/4_3/conditional.rn
The number *is* smaller than 5
== () (5.108ms)
```

Optionally, we can add another branch under `else`, which will execute in case
the condition is false.

```rust,noplaypen
{{#include ../../scripts/book/4_3/conditional_else.rn}}
```

```text
$> cargo run -- scripts/book/4_3/conditional_else.rn
the number is smaller than 5
== () (196.1µs)
```

We can also add an arbitrary number of `else if` branches, which allow us to
specify many different conditions.

```rust,noplaypen
{{#include ../../scripts/book/4_3/conditional_else_ifs.rn}}
```

```text
$> cargo run -- scripts/book/4_3/conditional_else_ifs.rn
the number is smaller than 5
== () (227.9µs)
```

Do note however that if you have *many* conditions, it might be cleaner to use
a `match`.

This will be covered in a later section, but here is a sneak peek:

```rust,noplaypen
{{#include ../../scripts/book/4_3/first_match.rn}}
```

```text
$> cargo run -- scripts/book/4_3/first_match.rn
the number is smaller than 5
== () (124.2µs)
```