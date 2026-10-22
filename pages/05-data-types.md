---
layout: section
---

<div class="chapter">Chapter 5</div>

# Structs, Enums & Pattern Matching

---
class: dense
---

# Structs & Methods

<div class="grid grid-cols-2 gap-8">
<div>

```rust {all|1-5|8-10|12-14|16-18|all}
#[derive(Debug)]
struct User {
    name: String,
    admin: bool,
}

impl User {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), admin: false }
    }

    fn greeting(&self) -> String {
        format!("Hello {}", self.name)
    }

    fn promote(&mut self) {
        self.admin = true;
    }
}
```

</div>
<div>

```rust {all|5-6|1|3-4|1-2|all}{at:1}
let mut kevin = User::new("Kevin");
kevin.promote();
println!("{}", kevin.greeting());
// Hello Kevin
println!("{kevin:?}");
// User { name: "Kevin", admin: true }
```

<ul class="mt-4">
  <li v-click="1">Data in the <code>struct</code>, methods in the <code>impl</code> block. <code>#[derive(Debug)]</code> generates the output with <code>{:?}</code>.</li>
  <li v-click="2"><code>new</code> has no <code>self</code>, so it is called with <code>User::new</code>. Methods with <code>self</code> are called with <code>.</code></li>
  <li v-click="3"><code>&amp;self</code> reads the instance (borrowing from chapter 4).</li>
  <li v-click="4"><code>&amp;mut self</code> changes it, so <code>kevin</code> must be <code>mut</code>.</li>
  <li v-click="5">No classes, no inheritance. Reuse through composition and traits.</li>
</ul>

</div>
</div>

---
class: dense
---

# The Newtype Pattern

So far, prices were a `u32` in cents and could be mixed up with quantities.

<div class="grid grid-cols-[9fr_11fr] gap-8 mt-2">
<div>

<div class="relative">

````md magic-move {at: 1}
```rust {all|1|3-5|8-9|all}
struct Money(u32); // amount in cents

fn total_price(price: Money, quantity: u32) -> Money {
    Money(price.0 * quantity)
}

fn main() {
    let total = total_price(Money(450), 3);
    println!("{} cents", total.0); // 1350 cents
}
```
```rust {8}
struct Money(u32); // amount in cents

fn total_price(price: Money, quantity: u32) -> Money {
    Money(price.0 * quantity)
}

fn main() {
    let total = total_price(3, Money(450));
    println!("{} cents", total.0);
}
```
````

<div class="code-pointer" v-click="5" style="top: 114px">▶</div>
</div>

<div class="text-base mt-2">

Tuple struct: fields without names, accessed with `.0`. A tuple struct with one field that wraps another type is called a newtype.

</div>

<div v-click="4" class="text-base mt-2">

`Money` takes up as much memory as a `u32`. The custom type has no runtime cost.

</div>

</div>
<div>

<div v-click="6" class="small-code">

```text {1|4-7|9-12|all}{at:7}
error[E0308]: arguments to this function are incorrect
 --> src/main.rs:8:17
  |
8 |     let total = total_price(3, Money(450));
  |                 ^^^^^^^^^^^ -  ---------- expected `u32`, found `Money`
  |                             |
  |                             expected `Money`, found `{integer}`
...
help: swap these arguments
  |
8 -     let total = total_price(3, Money(450));
8 +     let total = total_price(Money(450), 3);
```

</div>

<div v-click="9" class="text-base mt-4 muted">

The pattern exists in other languages too, e.g. `record struct` in C#, `value class` in Kotlin or `NewType` in Python.

</div>

</div>
</div>

---
class: dense
---

# Enums

Each variant can hold its own data.

<div class="grid grid-cols-2 gap-8 mt-2">
<div>

In Rust:

```rust {all|7-10|all}
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

let pending = OrderStatus::Pending;
let shipped = OrderStatus::Shipped {
    tracking_id: String::from("DE123"),
};
```

<div v-click="3" class="text-base mt-3">

A value of type `OrderStatus` is always exactly one of the variants, with exactly its data. The compiler checks that every variant is handled.

</div>

</div>
<div v-click="2">

In C#, without enums with data:

```csharp
class OrderStatus
{
    public string Kind;         // "Pending", "Shipped", ...
    public string? TrackingId;  // only for "Shipped"
    public string? Reason;      // only for "Cancelled"
}
```

<div class="text-base mt-3">

Nothing prevents `Kind = "Pending"` together with a `TrackingId`, or `"Shipped"` without one.

</div>

</div>
</div>

---
class: dense
---

# Pattern Matching with `match`

```rust {7-13|8|9|10|11|15-16|all}
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}

let status = OrderStatus::Shipped { tracking_id: String::from("DE123") };
println!("{}", describe(&status)); // shipped, tracking ID DE123
```

<div v-click="6" class="mt-4">

`match` must cover all variants. If a variant is added later, the compiler reports every `match` that does not handle it yet.

</div>

---
class: dense
---

# Missing Case in a `match`

<div class="relative small-code">

````md magic-move {at: 1}
```rust
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}
```
```rust {5}
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
    Returned { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}
```
````

<div class="code-pointer" v-click="1" style="top: 112px">▶</div>
</div>

<div v-click="2" class="small-code mt-3">

```text {1|2-5|6-8|9-12|all}{at:3}
error[E0004]: non-exhaustive patterns: `&OrderStatus::Returned { .. }` not covered
  --> src/main.rs:9:11
   |
 9 |     match status {
   |           ^^^^^^ pattern `&OrderStatus::Returned { .. }` not covered
...
 5 |     Returned { reason: String },
   |     -------- not covered
help: ensure that all possible cases are being handled by adding a match arm
      with a wildcard pattern or an explicit pattern as shown
   |
13 ~         &OrderStatus::Returned { .. } => todo!(),
```

</div>

---
class: dense
---

# Enums in Libraries: `#[non_exhaustive]`

````md magic-move
```rust
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}
```
```rust {1-3|9-12|17|all}
// library crate "shop"
#[non_exhaustive]
pub enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

// another crate
use shop::OrderStatus;

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
        _ => String::from("unknown status"),
    }
}
```
````

<div v-click="5" class="mt-4">

Other crates need a `_` arm, so new variants in the library do not break their code.

</div>

---
class: dense
---

# `Option<T>`

<div class="grid grid-cols-2 gap-8">
<div>

```rust
// from the standard library
enum Option<T> {
    Some(T),
    None,
}
```

```rust
fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User::new("Kevin"))
    } else {
        None
    }
}
```

</div>
<div>

```rust {all|1-4|6-8|10|all}
match find_user(42) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("No user"),
}

if let Some(user) = find_user(1) {
    println!("{}", user.greeting());
}

let user = find_user(7).unwrap_or(User::new("Guest"));
```

</div>
</div>

<div v-click class="mt-4 card">
Rust has no <code>null</code>. An optional value must be an <code>Option</code> in the type, and the <code>None</code> case must be handled.
</div>

---
class: dense
---

# `Result<T, E>`

<div class="grid grid-cols-2 gap-8">
<div>

```rust
// from the standard library
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
fn reserve(quantity: u32, in_stock: u32)
    -> Result<u32, String>
{
    if quantity > in_stock {
        Err(format!("only {in_stock} in stock"))
    } else {
        Ok(in_stock - quantity)
    }
}
```

</div>
<div>

```rust {all|1-4|6-8|all}
match reserve(3, 2) {
    Ok(left) => println!("Reserved, {left} left"),
    Err(message) => println!("Error: {message}"),
} // Error: only 2 in stock

if let Ok(left) = reserve(1, 10) {
    println!("Reserved, {left} left");
} // Reserved, 9 left
```

<div v-click="4" class="text-base mt-4">

In C#, the same function returns `int` and throws an exception. The signature does not show that it can fail.

</div>

</div>
</div>

<div v-click="5" class="mt-4 card">
Like <code>Option</code>, <code>Result</code> is an enum: either <code>Ok</code> with the result or <code>Err</code> with the error. Rust has no exceptions, possible errors are part of the return type.
</div>

---

# More Patterns

```rust
// should a failed call to the payment provider be retried?
fn should_retry(http_status: u16, attempt: u32) -> bool {
    match http_status {
        200..=299 => false,               // ranges
        429 => attempt < 5,               // single values
        408 | 503 if attempt < 3 => true, // alternatives with a guard
        _ => false,                       // everything else
    }
}
```

<div class="mt-6">

<v-clicks>

- Patterns for tuples, structs, slices and nested types
- `if let` for a single variant (see `Option`)
- `let ... else` for an early exit if the value does not match: `let Some(user) = find_user(7) else { return; };`
- `matches!(value, pattern)` returns `true` or `false`

</v-clicks>

</div>
