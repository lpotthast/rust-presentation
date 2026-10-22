---
layout: section
---

<div class="chapter">Chapter 6</div>

# Traits & Generics

<!--
- Shared behavior without inheritance. Copy and Debug from the previous chapters are already traits.
-->

---
class: dense
---

# Traits

Traits define shared behavior of types, similar to interfaces.

<div class="grid grid-cols-2 gap-8">
<div>

```rust {all|1-8|10-14|all}
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

struct Console;
```

<div v-click="7" class="mt-4 muted text-base">
Your own traits can also be implemented for foreign types such as <code>String</code> or <code>i32</code>.
</div>

</div>
<div v-click="3">

```rust {all|1-5|7-16|all}{at:4}
impl Notifier for Email {
    fn recipient(&self) -> &str {
        &self.address
    }
}

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

</div>
</div>

<!--
- Example: notifications over different channels
- struct Console; is a struct without fields
- Right side: Email only implements recipient and uses the default notify. Console also overrides notify.
- Click 3: the implementations appear once the trait and the structs are known
- recipient returns a &str borrowed from self, "stdout" is a string literal. No lifetime annotation needed, the compiler infers it (Lifetime Annotations, chapter 4).
- Unlike Java interfaces, the implementation is not part of the type definition
- Existing types can implement traits later
- Orphan rule: the trait or the type must come from your own crate. A foreign trait such as Display cannot be implemented for a foreign type such as Vec.
-->

---
class: dense
---

# Generics & Trait Objects

Two variants of the same function. Only the parameter type differs.

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

<!--
- Dispatch: choosing the method to call, at compile time (static) or at runtime (dynamic)
- Both columns show alternative versions of the same function. Only the parameter type differs: &impl Notifier vs. &dyn Notifier. The calls and their output are identical.
- The output shows the Traits slide again: Console overrides notify, Email uses the default implementation.
- Click 1: impl Notifier. Click 2: dyn Notifier, a trait object.
- Click 3: A Vec holds values of a single type. With &dyn Notifier, Console and Email fit into the same list. With impl or generics this is not possible.
- vtable: like virtual methods in C++ or Java. Java: always dynamic, C++: dynamic only with virtual, Rust: explicit choice
- For a list that owns its elements: Vec<Box<dyn Notifier>>. Box<T> puts a value on the heap, more smart pointers in chapter 8.
- Complete code: cargo run --example 06_traits
-->

---
class: dense
---

# Standard Traits & `derive`

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|1-2|4-8|10-12|all}
#[derive(Debug, Clone, Copy, PartialEq)]
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

</div>
<div>

| Trait                    | Meaning                |
| ------------------------ | ---------------------- |
| `Debug`                  | output with `{:?}`     |
| `Display`                | output with `{}`       |
| `Clone`                  | copy with `clone()`    |
| `Copy`                   | implicit copy          |
| `PartialEq`              | comparison with `==`   |
| `From` / `Into`          | conversion between types |

</div>
</div>

<!--
- Optional
- The import is left out for space: use std::fmt; (complete in code-examples/examples/06_std_traits.rs)
- Money is the newtype from chapter 5, amounts in cents to avoid rounding errors
- Before: println!("{} cents", total.0) on the Newtype slide. With Display: println!("{price}").
- Click 1: derive line and struct together, with the derive sentence. Click 2: Display, written by hand, with the Display sentence. Click 3: usage.
- fmt::Result is an alias for Result<(), fmt::Error>, Result was introduced in chapter 5
- Other common traits: Hash + Eq (keys in a HashMap), Default (default value), Iterator
- Operators like + are overloaded via traits from std::ops, e.g. Add
- Libraries provide their own derive macros, e.g. serde (chapter 9)
-->

---
class: dense
---

# Iterators & Closures

Similar to Java Streams or C# LINQ. Closures are like lambdas or arrow functions.

```rust {all|1-7|9-13|15|all}
let lines = [
    "INFO server started",
    "ERROR disk full",
    "INFO request handled",
    "ERROR connection lost",
];
let level = "ERROR"; // captured by the closure

let alerts: Vec<String> = lines
    .iter()
    .filter(|line| line.starts_with(level))
    .map(|line| line.to_lowercase())
    .collect(); // ["error disk full", "error connection lost"]

let info_count = lines.iter().filter(|line| line.starts_with("INFO")).count(); // 2
```

<div v-click class="card mt-3">
<code>Iterator</code> is a trait with the required method <code>next()</code>. <code>filter</code>, <code>map</code> and <code>count</code> are default methods.
Iterators are lazy and only run on a consuming method such as <code>collect()</code>. The chain compiles to code as fast as a hand-written loop.
</div>

<!--
- Example: filtering error messages from log lines
- filter, map and count are default methods of the Iterator trait, like notify on the Traits slide. Implementing next() is enough to get all of them.
- Zero-cost abstraction: the chain compiles to the same machine code as a manual loop
- Closure syntax: |parameter| expression, similar to lambdas in Java/C# or arrow functions in TypeScript
- Closures can use variables from their environment (here level)
- collect() gathers the results, the target type is on the left: Vec<String>
- rayon: .par_iter() instead of .iter() runs the chain in parallel
-->
