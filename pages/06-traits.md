---
layout: section
---

<div class="chapter">Chapter 6</div>

# Traits & Generics

---
class: dense
---

# Traits

Traits define shared behavior of types, similar to interfaces.

<div class="grid grid-cols-2 gap-8">
<div>

```rust {all|1-8|10-18|none|all}
trait Notifier {
    fn recipient(&self) -> &str;

    // default implementation
    fn notify(&self, message: &str) {
        println!("To {}: {message}", self.recipient());
    }
}

struct Email {
    address: String,
}

impl Notifier for Email {
    fn recipient(&self) -> &str {
        &self.address
    }
}
```

</div>
<div v-click="3">

```rust
struct Console;

impl Notifier for Console {
    fn recipient(&self) -> &str {
        "stdout"
    }

    // override the default implementation
    fn notify(&self, message: &str) {
        println!("[console] {message}");
    }
}
```

<div v-click="4" class="mt-4 muted text-base">
Your own traits can also be implemented for foreign types such as <code>String</code> or <code>i32</code>.
</div>

</div>
</div>

---
class: dense
---

# `Drop`

A standard trait. The compiler calls its method when a value is dropped.

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|5-10|13-16|all}
struct Reservation {
    item: String,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        // e.g. put the item back in stock
        println!("released {}", self.item);
    }
}

fn main() {
    {
        let r = Reservation { item: String::from("Banana") };
        println!("reserved {}", r.item);
    } // the compiler calls drop here
    println!("done");
}
```

</div>
<div class="text-base">

<div v-click="1">

`Drop` has a single method, `drop`. It runs when the owner goes out of scope (Ownership Rules, chapter 4).

</div>

<div v-click="2" class="mt-4">

```text
reserved Banana
released Banana
done
```

</div>

<div v-click="3" class="mt-4">

Typical uses: closing files and connections, releasing locks, returning resources.

</div>

<div v-click="3" class="mt-4">

`r.drop()` does not compile. The function `drop(r)` moves the value and drops it early.

</div>

</div>
</div>

---
class: dense
---

# Generics & Trait Objects

<div class="grid grid-cols-2 gap-8">
<div>

**Static dispatch** (default)

```rust {all|1}
fn alert(notifier: &impl Notifier) {
    notifier.notify("disk almost full");
}

let email = Email { address: String::from("ops@shop.de") };
alert(&Console); // [console] disk almost full
alert(&email);   // To ops@shop.de: disk almost full
```

```rust
fn alert<T: Notifier>(notifier: &T) // same, generic syntax
```

<div class="muted text-base">

`T: Notifier` is a trait bound: `T` must implement `Notifier`. The compiler generates one version per type, `alert::<Console>` and `alert::<Email>` (monomorphization), with no runtime overhead.

</div>

</div>
<div v-click="2">

**Dynamic dispatch**

```rust {1}
fn alert(notifier: &dyn Notifier) {
    notifier.notify("disk almost full");
}

let email = Email { address: String::from("ops@shop.de") };
alert(&Console); // [console] disk almost full
alert(&email);   // To ops@shop.de: disk almost full
```

<div class="muted text-base">

`&dyn Notifier` is a trait object. There is only one version of `alert`, the method is looked up at runtime through a vtable.

</div>

<div v-click="3" class="mt-2">

`dyn` is needed when different types are stored together:

```rust
let notifiers: Vec<&dyn Notifier> = vec![&Console, &email];
```

</div>

</div>
</div>

---
class: dense
---

# Flexible Parameters with `impl Into`

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

````md magic-move
```rust
struct User {
    name: String,
    admin: bool,
}

impl User {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), admin: false }
    }
}

let kevin = User::new("Kevin");
let input = String::from("Stuart");           // e.g. from a request
let stuart = User::new(&input);               // the text is copied
```
```rust {7-8,12,14}
struct User {
    name: String,
    admin: bool,
}

impl User {
    fn new(name: String) -> Self {
        Self { name, admin: false }
    }
}

let kevin = User::new(String::from("Kevin")); // conversion needed
let input = String::from("Stuart");           // e.g. from a request
let stuart = User::new(input);                // moved, no copy
```
```rust {7-8|12-14|all}
struct User {
    name: String,
    admin: bool,
}

impl User {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), admin: false }
    }
}

let kevin = User::new("Kevin");               // &str is converted
let input = String::from("Stuart");           // e.g. from a request
let stuart = User::new(input);                // moved, no copy
```
````

<div v-click="4">

```rust
fn new<S: Into<String>>(name: S) -> Self // same, generic syntax
```

</div>

</div>
<div class="text-base">

<div class="grid into-swap">
<div v-click-hide="1" style="grid-area: 1 / 1">

`new` from the slide Structs & Methods takes a `&str`. A literal can be passed directly. `input` already owns a `String`, but `new` copies the text into a new `String`.

</div>
<div v-click="[1, 2]" style="grid-area: 1 / 1">

With a `String` parameter, `input` is moved into the struct without a copy. Every call with a literal now needs `String::from`.

</div>
<div v-click="2" style="grid-area: 1 / 1">

`impl Into<String>` accepts every type that can be converted into a `String`. Same `impl` syntax as `&impl Notifier` on the previous slide. `name.into()` performs the conversion.

</div>
</div>

<div v-click="3" class="mt-4">

Both calls are written as with `&str`, and `input` is still moved without a copy.

</div>

<div v-click="4" class="mt-4">

`Into` is a standard trait. `String` implements `From<&str>`, `From<char>` and more, and every `From` implementation provides the matching `Into` automatically.

</div>

</div>
</div>

<style>
.into-swap p { margin-bottom: 0; }
</style>

---
class: dense
---

# Standard Traits & `derive`

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|1-2|4-8|10-12|all}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Money(u32); // amount in cents

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} EUR", self.0 / 100, self.0 % 100)
    }
}

let price = Money(1649);
println!("{price}");   // 16.49 EUR
println!("{price:?}"); // Money(1649)
```

<div class="muted text-base mt-4">
<span v-click="1"><code>derive</code> generates the implementation at compile time.</span>
<span v-click="2"><code>Display</code> defines how the value looks to users and is written by hand.</span>
</div>

<style>
td:first-child { white-space: nowrap; }
</style>

</div>
<div>

| Trait                    | Meaning                |
| ------------------------ | ---------------------- |
| `Debug`                  | output with `{:?}`     |
| `Display`                | output with `{}`       |
| `Clone`                  | copy with `clone()`    |
| `Copy`                   | implicit copy          |
| `PartialEq`, `Eq`        | comparison with `==`   |
| `PartialOrd`, `Ord`      | order with `<`, `>`    |
| `From` / `Into`          | conversion between types |

</div>
</div>

---
class: dense
---

# Equality & Ordering

<div class="grid grid-cols-2 gap-8 mt-2">
  <div class="card">
    <h3 class="c-orange"><code>PartialEq</code> and <code>Eq</code></h3>
    <ul>
      <li><code>PartialEq</code>: <code>==</code> and <code>!=</code>, symmetric and transitive</li>
      <li><code>Eq</code>: a marker trait without methods. Promises that every value is equal to itself (<code>a == a</code>).</li>
      <li><code>f64</code> is only <code>PartialEq</code>: <code>NaN == NaN</code> is <code>false</code></li>
      <li><code>HashMap</code> keys need <code>Eq</code> and <code>Hash</code></li>
    </ul>
  </div>
  <div class="card" v-click="1">
    <h3 class="c-orange"><code>PartialOrd</code> and <code>Ord</code></h3>
    <ul>
      <li><code>PartialOrd</code>: <code>&lt;</code>, <code>&lt;=</code>, <code>&gt;</code>, <code>&gt;=</code>, transitive</li>
      <li><code>Ord</code>: adds the method <code>cmp</code>. Any two values can be compared (total order).</li>
      <li><code>f64</code> is only <code>PartialOrd</code>: <code>NaN</code> is neither smaller, equal nor larger than <code>1.0</code></li>
      <li><code>sort()</code> and <code>BTreeMap</code> keys need <code>Ord</code></li>
    </ul>
  </div>
</div>

<div v-click="2" class="mt-4">

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Money(u32); // derive works because u32 has all four traits

let nan = f64::NAN;
```

</div>

<div class="grid grid-cols-2 gap-8 mt-2" v-click="3">
<div>

```rust
nan == nan;                // false
Money(450) == Money(450);  // true for every Money
```

</div>
<div>

```rust
nan.partial_cmp(&1.0);         // None
Money(450).cmp(&Money(1299));  // Ordering::Less
```

</div>
</div>

---
class: dense
---

# Iterators & Closures

Similar to Java Streams or C# LINQ. Closures are like lambdas or arrow functions.

```rust {all|1-2|4-5|6|7|8|all}
let input = "summer, sale,, kids , extra-waterproof"; // e.g. from a form field
let max_len = 10; // captured by the closure

let tags: Vec<&str> = input
    .split(',')
    .map(|tag| tag.trim())
    .filter(|tag| !tag.is_empty() && tag.len() <= max_len)
    .collect(); // ["summer", "sale", "kids"]
```

<div v-click="7" class="grid grid-cols-[2fr_3fr] gap-6 mt-3 items-center">
<div>

```text
input  "summer, sale,, kids , extra-waterproof"
tags    ^^^^^^  ^^^^   ^^^^
```

</div>
<div class="text-base">

`split` and `trim` return `&str` slices that point into `input`. No text is copied, the only heap allocation is the `Vec`. With `.count()` instead of `.collect()`, the chain allocates nothing.

</div>
</div>

<div v-click="8" class="card mt-3">
<code>map</code>, <code>filter</code> and <code>collect</code> are default methods of the <code>Iterator</code> trait, which only requires <code>next()</code>.
The chain compiles to code as fast as a hand-written loop.
</div>
