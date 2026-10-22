---
layout: section
---

<div class="chapter">Chapter 3</div>

# Syntax Basics

---

# Hello World & Variables

<div class="grid grid-cols-2 gap-8">
<div>

```rust
fn main() {
    println!("Hello, world!");
}
```

<div class="muted text-base mt-2">
<code>println!</code> is a macro, recognizable by the <code>!</code>.
</div>

</div>
<div>

```rust {all|1-2|3-4|5|6-7|all}
let max_items = 10;       // inferred: i32
// max_items = 20;        // error: immutable
let mut cart_items = 0;   // mutable
cart_items += 1;
let price: u32 = 450;     // explicit, otherwise i32
let input = "  Banana  ";
let input = input.trim(); // shadowing
```

<div v-click="1" class="mt-4">
Variables are immutable by default. <code>mut</code> makes them mutable.
</div>

</div>
</div>

---
class: dense
---

# Data Types

<div class="grid grid-cols-2 gap-8 mt-4">
<div>

| Kind            | Types                                |
| --------------- | ------------------------------------ |
| Signed integers   | `i8` `i16` `i32` `i64` `i128` `isize` |
| Unsigned integers | `u8` `u16` `u32` `u64` `u128` `usize` |
| Floating point  | `f32` `f64`                          |
| Boolean         | `bool`                               |
| Character       | `char` (Unicode, 4 bytes)            |
| Text            | `String` and `&str`                  |

<div class="muted text-base mt-4">

`usize` is used for indices and lengths. No implicit conversions between number types: `i32 + i64` does not compile without an explicit conversion (`as`).

</div>

</div>
<div>

```rust
// tuple
let item: (&str, u32) = ("Banana", 450);
let (name, price) = item;

// array: fixed length
let sizes: [&str; 3] = ["S", "M", "L"];

// Vec: growable (heap)
let mut cart = vec![450, 1200];
cart.push(299);

// HashMap
use std::collections::HashMap;
let mut stock = HashMap::new();
stock.insert("Banana", 12);
```

</div>
</div>

---

# Functions & Expressions

```rust {all|1-3|7|all}
fn total_price(price: u32, quantity: u32) -> u32 {
    price * quantity // no semicolon: this is the return value
}

fn main() {
    let total = total_price(450, 3);
    let shipping = if total >= 1000 { 0 } else { 495 };
    println!("total: {total}, shipping: {shipping}"); // total: 1350, shipping: 0
}
```

<div v-click="1">

- Almost everything is an expression and produces a value, e.g. blocks `{ }`
- The last expression without `;` is the value of the block
- Parameter and return types are always written out, there is no inference in signatures

</div>
<div v-click="2">

- `if` is an expression as well, so there is no ternary operator

</div>

---
class: dense
---

# Loops

<div class="grid grid-cols-2 gap-8">
<div>

```rust
// range: 0 to 4
for i in 0..5 {
    println!("{i}");
}

let cart = ["Banana", "Goggles", "Overalls"];

// iterate over a collection
for item in cart {
    println!("{item}");
}

// with index: 1. Banana, 2. Goggles, ...
for (i, item) in cart.iter().enumerate() {
    println!("{}. {item}", i + 1);
}
```

</div>
<div>

```rust
let mut stock = 3;

// while: repeats as long as the condition holds
while stock > 0 {
    println!("sold one, {stock} left");
    stock -= 1;
}

// loop: runs until break, break can return a value
let mut attempt = 0;
let delay_ms = loop {
    attempt += 1;
    if attempt == 3 {
        break attempt * 100; // retry delay
    }
};
```

</div>
</div>

<div v-click class="mt-6">

`for` always runs over an iterator, e.g. a range or a collection. There is no counting loop like `for (i = 0; i < n; i++)`.

</div>

---
class: dense
---

# Modules & Visibility

<div class="grid grid-cols-2 gap-8">
<div>

<div class="relative">

```rust
// src/main.rs
mod pricing; // includes src/pricing.rs

fn main() {
    let total = pricing::order_total(450, 3);
    pricing::shipping_cost(total);
    println!("{total}");
}
```

<div class="code-pointer" style="top: 88px">▶</div>
</div>

<div class="grid mt-3">
<div v-click="[2, 5]" class="small-code" style="grid-area: 1 / 1">

```text {1|2-5|7-11}{at:3}
error[E0603]: function `shipping_cost` is private
 --> src/main.rs:6:14
  |
6 |     pricing::shipping_cost(total);
  |              ^^^^^^^^^^^^^ private function
  |
note: the function `shipping_cost` is defined here
 --> src/pricing.rs:7:1
  |
7 | fn shipping_cost(subtotal: u32) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

</div>
<div v-click="5" class="text-base" style="grid-area: 1 / 1">

Everything is private by default, `pub` makes items visible to other modules.

Modules can be separate files or inline with `mod name { ... }`.

</div>
</div>

</div>
<div>

```text
src/
├── main.rs
└── pricing.rs
```

```rust {all|1-9|1-9|1-9|7|11-18|all}
// src/pricing.rs
pub fn order_total(price: u32, quantity: u32) -> u32 {
    let subtotal = price * quantity;
    subtotal + shipping_cost(subtotal)
}

fn shipping_cost(subtotal: u32) -> u32 {
    if subtotal >= 1000 { 0 } else { 495 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_shipping_from_10_euros() {
        assert_eq!(shipping_cost(1000), 0);
    }
}
```

</div>
</div>

---
class: dense
---

# Tests

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

````md magic-move
```rust
pub fn is_valid_port(port: u16) -> bool {
    port >= 1024
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*; // dev-dependency

    #[test]
    fn rejects_reserved_ports() {
        // standard library: assert!(!is_valid_port(80));
        assert_that!(is_valid_port(80)).is_false();
    }
}
```
```rust {3-8|all}
/// Returns true for ports that need no root privileges.
///
/// ```
/// use assertr::prelude::*;
/// use my_crate::is_valid_port;
/// assert_that!(is_valid_port(1023)).is_false();
/// assert_that!(is_valid_port(1024)).is_true();
/// ```
pub fn is_valid_port(port: u16) -> bool {
    port >= 1024
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*; // dev-dependency

    #[test]
    fn rejects_reserved_ports() {
        // standard library: assert!(!is_valid_port(80));
        assert_that!(is_valid_port(80)).is_false();
    }
}
```
````

</div>
<div class="text-base">

Unit tests in the same file, integration tests in `tests/`, run with `cargo test`. The test runner is built in.

`assert!` and `assert_eq!` are built in, `assert_that!` comes from assertr.

<div v-click="1" class="mt-4">

**Doc tests:** code examples in the documentation are run by `cargo test` as well.

</div>

<div v-click="2" class="mt-4">

```text
$ cargo test
running 1 test
test tests::rejects_reserved_ports ... ok

   Doc-tests my_crate
running 1 test
test src/lib.rs - is_valid_port (line 3) ... ok
```

</div>

</div>
</div>

<style>
.slidev-code:has(.slidev-code-dishonored) .slidev-code-highlighted span[style*="727072"] { color: #b5b3b5 !important; }
</style>
