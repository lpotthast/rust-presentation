---
layout: section
---

<div class="chapter">Chapter 4</div>

# Ownership & Borrowing

---
class: dense
---

# Stack & Heap

<div class="grid grid-cols-[2fr_3fr] gap-8 mt-2">
<div class="flex flex-col gap-3 compact">
  <div class="card">
    <h3 class="c-blue">Stack</h3>
    <p>One area per function call, for values whose size is known at compile time. Allocating only moves a pointer. The values are freed automatically when the function returns.</p>
  </div>
  <div class="card">
    <h3 class="c-yellow">Heap</h3>
    <p>For data whose size is only known at runtime or that grows. An allocator reserves the memory, and it must be freed again later. Who frees it, and when, is the topic of this chapter.</p>
  </div>
</div>
<div>

```rust {all|1-3|4|5|all}
let price: u32 = 450;         // stack
let item = ("Banana", 450);   // stack (tuple)
let sizes = ["S", "M", "L"];  // stack (array)
let cart = vec![450, 1200];   // ptr, len, cap on the stack, elements on the heap
let boxed = Box::new(450);    // the number on the heap
```

<div v-click="3" class="text-base mt-3">

`Box<T>` puts a single value on the heap. `Vec`, `String` and `Box` keep only a pointer and sizes on the stack.

</div>

</div>
</div>

<div v-click="4" class="mt-6">

By default, every value is on the stack, including structs, tuples and arrays. Only types such as `Box`, `Vec`, `String` or `Arc` store data on the heap. In Java, every object is on the heap. In C#, instances of a `class` are, a `struct` is not.

</div>

---
class: dense
---

# Memory Management

The same task in three languages: create a text on the heap, use it, free it.

<div class="grid grid-cols-3 gap-6 mt-4 compact">
  <div class="card flex flex-col" v-click>
    <h3 class="c-red">Manual</h3>
    <p class="mb-2">C (malloc/free), C++ (new/delete)</p>

<div class="small-code">

```c
char *name = malloc(6);
strcpy(name, "hello");
puts(name);
free(name); // by hand
```

</div>

<p class="mt-3">Memory is allocated and freed explicitly.</p>
    <p style="margin-top: auto; padding-top: 0.5rem"><span class="c-green">✓</span> fast, predictable<br><span class="c-red">✘</span> memory leaks, use-after-free, double free</p>
  </div>
  <div class="card flex flex-col" v-click>
    <h3 class="c-yellow">Garbage Collector</h3>
    <p class="mb-2">Java, C#, Go, Python, JS</p>

<div class="small-code">

```java
var name = new StringBuilder("hello");
System.out.println(name);
// freed later by the GC
```

</div>

<p class="mt-3">The runtime frees memory that is no longer reachable.</p>
    <p style="margin-top: auto; padding-top: 0.5rem"><span class="c-green">✓</span> safe, simple<br><span class="c-red">✘</span> higher memory usage, pauses</p>
  </div>
  <div class="card flex flex-col" v-click style="border-color: var(--mp-orange)">
    <h3 class="c-orange">Ownership</h3>
    <p class="mb-2">Rust</p>

<div class="small-code">

```rust
{
    let name = String::from("hello");
    println!("{name}");
} // freed here (drop)
```

</div>

<p class="mt-3">The compiler inserts the deallocation at the end of the owner's scope.</p>
    <p style="margin-top: auto; padding-top: 0.5rem"><span class="c-green">✓</span> safe, fast, predictable<br><span class="c-red">✘</span> learning effort</p>
  </div>
</div>

---

# Ownership Rules

<span class="c-orange font-600">Variable</span>: a name in the code. <span class="c-orange font-600">Value</span>: the data the variable refers to.

<div class="grid grid-cols-2 gap-8 mt-4">
<div class="flex flex-col gap-3">
  <div class="card" v-click="1"><span class="c-orange font-700 inline-block w-7">1.</span> Each value has exactly one owner, usually a variable.</div>
  <div class="card" v-click="2"><span class="c-orange font-700 inline-block w-7">2.</span> When the owner goes out of scope, the value is dropped (<code>drop</code>).</div>
</div>
<div v-click="3">

````md magic-move {at: 4}
```rust
fn main() {
    let order = String::from("A-1001");
    {
        let label = format!("Label {order}");
        println!("{label}");
    }
    println!("{order}");
}
```
```rust {6,9}
fn main() {
    let order = String::from("A-1001");
    {
        let label = format!("Label {order}");
        println!("{label}");
        drop(label); // inserted by the compiler
    }
    println!("{order}");
    drop(order); // inserted by the compiler
}
```
````

</div>
</div>

---

# Move

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div class="relative">

<div class="code-pointer" :style="{ top: (24 + ([2, 3, 7, 8, 14, 12][Math.min($clicks, 5)] - 1) * 18) + 'px' }">▶</div>

````md magic-move
```rust
fn main() {
    let s1 = String::from("hello");
}
```
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 is now the owner (move)
    // println!("{s1}"); // error: s1 is no longer valid
    println!("{s2}");
} // s2 is dropped here
```
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 is now the owner (move)
    // println!("{s1}"); // error: s1 is no longer valid
    println!("{s2}");

    let s3 = s2.clone(); // explicit deep copy
} // s2 and s3 are dropped here
```
```rust {all|12-14}
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 is now the owner (move)
    // println!("{s1}"); // error: s1 is no longer valid
    println!("{s2}");

    let s3 = s2.clone(); // explicit deep copy
    take(s3); // the parameter s becomes the owner
    // s3 is no longer valid here
} // s2 is dropped here

fn take(s: String) {
    println!("{s}");
} // s is dropped here
```
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 is now the owner (move)
    // println!("{s1}"); // error: s1 is no longer valid
    println!("{s2}");

    let s3 = s2.clone(); // explicit deep copy
    take(s3); // the parameter s becomes the owner
    // s3 is no longer valid here

    let a = 42;
    let b = a; // i32 is Copy and is simply copied
    println!("{a} {b}");
} // s2 is dropped here

fn take(s: String) {
    println!("{s}");
} // s is dropped here
```
````

</div>
<div class="text-base">

<div class="card memgrid">
  <div class="c-blue font-600">Stack</div><div></div><div class="c-yellow font-600">Heap</div>
  <div class="grid">
    <code v-click-hide="1" style="grid-area: 1 / 1">s1 = { ptr, len: 5, cap: 5 }</code>
    <code v-click="1" class="opacity-50" style="grid-area: 1 / 1">s1 <span class="muted">(moved)</span></code>
  </div>
  <div class="mem-arrow" v-click-hide="1" style="color: var(--mp-fg); font-size: 1.15em">→</div>
  <code>"hello"</code>
  <code v-click="1">s2 = { ptr, len: 5, cap: 5 }</code>
  <div class="mem-arrow" v-click="1" style="color: var(--mp-fg); font-size: 1.15em">↗</div>
  <div></div>
  <div class="grid" v-click="2">
    <code v-click-hide="3" style="grid-area: 1 / 1">s3 = { ptr, len: 5, cap: 5 }</code>
    <code v-click="3" class="opacity-50" style="grid-area: 1 / 1">s3 <span class="muted">(moved)</span></code>
  </div>
  <div class="mem-arrow" v-click="[2, 3]" style="color: var(--mp-fg); font-size: 1.15em">→</div>
  <div class="grid" v-click="2">
    <code v-click-hide="4" style="grid-area: 1 / 1">"hello"</code>
    <code v-click="4" class="c-red line-through" style="grid-area: 1 / 1">"hello"</code>
  </div>
  <div class="grid" v-click="3">
    <code v-click-hide="4" style="grid-area: 1 / 1">s  = { ptr, len: 5, cap: 5 } <span class="muted">(take)</span></code>
    <code v-click="4" class="c-red line-through" style="grid-area: 1 / 1">s  = { ptr, len: 5, cap: 5 } <span class="muted">(take)</span></code>
  </div>
  <div class="mem-arrow" v-click="[3, 4]" style="color: var(--mp-fg); font-size: 1.15em">↗</div>
  <div></div>
  <code v-click="5">a = 42</code><div></div><div v-click="5" class="muted text-sm whitespace-nowrap">no heap</div>
  <code v-click="5">b = 42</code><div></div><div></div>
</div>

<div class="grid muted mt-4">
  <div v-click="[1, 2]" style="grid-area: 1 / 1">Only <code>ptr</code>, <code>len</code> and <code>cap</code> are copied. If <code>s1</code> stayed valid, both would free "hello" (double free), so <code>s1</code> becomes invalid. In Java, both would refer to the same object.</div>
  <div v-click="[2, 3]" style="grid-area: 1 / 1"><code>clone()</code> creates a second value with its own copy on the heap, owned by <code>s3</code>.</div>
  <div v-click="[3, 4]" style="grid-area: 1 / 1"><code>take(s3)</code> moves the value into <code>s</code>. The data stays unchanged on the heap.</div>
  <div v-click="[4, 5]" style="grid-area: 1 / 1">At the end of <code>take</code>, <code>s</code> goes out of scope. Only now is the value dropped and its memory freed.</div>
  <div v-click="5" style="grid-area: 1 / 1">Numbers are <code>Copy</code>. <code>b</code> gets a copy of the value, everything lives on the stack.</div>
</div>

</div>
</div>

---

# Use After Move

<div class="relative">
<div class="code-pointer" style="top: 66px">▶</div>

```rust {all|2|3|4|3|all}{at:2}
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
}
```

</div>

<div v-click="1" class="small-code mt-4">

```text {1-2|4-5|6-7|8-9|11-14|all}{at:2}
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{s1}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

</div>

---

# Borrowing

<div class="relative">

````md magic-move
```rust
fn length(text: String) -> usize { // takes ownership
    text.len()
}

fn main() {
    let address = String::from("Banana Street 1");
    let n = length(address);
    println!("{address} ({n})"); // error: address was moved
}
```
```rust {1,7}
fn length(text: &String) -> usize { // borrows for reading
    text.len()
}

fn main() {
    let address = String::from("Banana Street 1");
    let n = length(&address);
    println!("{address} ({n})"); // Banana Street 1 (15)
}
```
```rust {5-7,10-11|all}
fn length(text: &String) -> usize { // borrows for reading
    text.len()
}

fn append(text: &mut String) { // borrows for writing
    text.push_str(", Minion Town");
}

fn main() {
    let mut address = String::from("Banana Street 1");
    append(&mut address);
    let n = length(&address);
    println!("{address} ({n})"); // Banana Street 1, Minion Town (28)
}
```
````

<div class="code-pointer" v-click-hide="1" style="top: 138px">▶</div>
</div>

<div v-click="3" class="mt-4">

`&address` and `&mut address` are references to the value of `address`. The variable `address` stays the owner of the value.

</div>

---

# Borrowing Rules

<div class="grid grid-cols-2 gap-8 mt-4">
<div>

<div class="card text-lg">
At any time, <strong>either</strong>:
<ul class="mt-2">
<li>any number of <code>&T</code> (read)</li>
<li><strong>or</strong> exactly one <code>&mut T</code> (write)</li>
</ul>
</div>

<div class="card text-lg mt-4">
References always point to a valid value.
</div>

<div v-click="7" class="mt-6 muted">
<code>push</code> needs <code>&mut cart</code> while <code>first</code> still holds <code>&cart[0]</code>. If <code>push</code> were allowed, the vector could move its elements to a new, larger allocation. <code>first</code> would then point to freed memory, and <code>println!</code> would read it.
</div>

</div>
<div>

<div v-click="1" class="relative">

````md magic-move {at: 3}
```rust {all|3|4|5|all|all}
fn main() {
    let mut cart = vec!["Banana", "Goggles"];
    let first = &cart[0];
    cart.push("Overalls");
    println!("{first}");
}
```
```rust {4-5}
fn main() {
    let mut cart = vec!["Banana", "Goggles"];
    let first = &cart[0];
    println!("{first}");
    cart.push("Overalls");
}
```
````

<div class="code-pointer" v-click-hide="8" style="top: 66px">▶</div>
</div>

<div class="grid mt-3">
<div v-click="[2, 8]" class="small-code" style="grid-area: 1 / 1">

```text {1-2|5-6|7-8|9-10|all}{at:3}
error[E0502]: cannot borrow `cart` as mutable
              because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &cart[0];
  |                  ---- immutable borrow occurs here
4 |     cart.push("Overalls");
  |     ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{first}");
  |                ----- immutable borrow later used here
```

</div>
<div v-click="8" class="text-base" style="grid-area: 1 / 1">

This version compiles. The borrow held by `first` ends at its last use, after that `push` may change `cart`.

</div>
</div>

</div>
</div>

---
class: dense
---

# `String` and `&str`

```rust {all|1|2|3|all}
let address = String::from("Banana Street 1"); // owns the text on the heap
let street: &str = &address[0..6];             // string slice: "Banana"
let city = "Minion Town";                      // literal: &'static str
```

<div class="grid grid-cols-[3fr_2fr] gap-8 mt-4">
<div>

<div class="card memgrid str-mem" v-click="1">
  <div class="c-blue font-600">Stack</div><div></div><div class="c-yellow font-600">Data</div><div></div>
  <code>address = { ptr, len: 15, cap: 15 }</code>
  <div class="mem-arrow">→</div>
  <code>"<span :class="{ 'c-orange': $clicks >= 2 }">Banana</span> Street 1"</code>
  <div class="muted text-sm">heap</div>
  <code v-click="2">street  = { ptr, len: 6 }</code>
  <div v-click="2" class="mem-arrow">↗</div>
  <div></div><div></div>
  <code v-click="3">city    = { ptr, len: 11 }</code>
  <div v-click="3" class="mem-arrow">→</div>
  <code v-click="3">"Minion Town"</code>
  <div v-click="3" class="muted text-sm">binary</div>
</div>

</div>
<div v-click="4">

| Use                    | Type           |
| ---------------------- | -------------- |
| Read-only parameter    | `&str`         |
| Struct field           | `String`       |
| Newly built text       | `String`       |
| Literal                | `&'static str` |

</div>
</div>

<div v-click="5" class="mt-4">

Text is always UTF-8. `"Müsli".len()` is 6 bytes, `.chars()` iterates over the 5 characters. `address[0]` does not compile.

</div>

<style>
.str-mem { grid-template-columns: auto 1.6rem auto auto; }
.str-mem .mem-arrow { color: var(--mp-fg); font-size: 1.15em; }
</style>

---
class: dense
---

# Owned & Borrowed Types

<div class="grid grid-cols-[2fr_3fr] gap-8 mt-2">
<div>

| Owned       | Borrowed    | Content |
| ------------------------- | ------------------- | ------ |
| `String`                  | `&str`              | UTF-8 text |
| `PathBuf`                 | `&Path`             | file paths |
| `Vec<T>`                  | `&[T]`              | lists (slice) |

<div v-click="1" class="mt-4 text-base">

Owned types own their data on the heap. Borrowed types are references to that data or to literals in the program.

</div>

</div>
<div v-click="2">

```rust
fn total(prices: &[u32]) -> u32 {
    prices.iter().sum() // iterators: chapter 6
}

let cart: Vec<u32> = vec![450, 1299];
total(&cart);        // &Vec<u32> -> &[u32]
total(&[450, 1299]); // array, also a &[u32]
```

<div class="mt-2 text-base">

Functions usually take the borrowed variant (`&str`, `&[T]`) and therefore accept both forms.

</div>

</div>
</div>

---
class: dense
---

# Lifetimes

A reference must not live longer than the value it points to.

<div class="relative">
<div class="code-pointer" style="top: 72px">▶</div>

```rust {all|4|5|6|7|all}{at:2}
fn main() {
    let r;
    {
        let s = String::from("hello");
        r = &s;
    } // s is dropped here
    println!("{r}");
}
```

</div>

<div v-click="1" class="small-code mt-4">

```text {1|4-5|6-7|8-9|10-11|all}{at:2}
error[E0597]: `s` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let s = String::from("hello");
  |             - binding `s` declared here
5 |         r = &s;
  |             ^^ borrowed value does not live long enough
6 |     } // s is dropped here
  |     - `s` dropped here while still borrowed
7 |     println!("{r}");
  |                - borrow later used here
```

</div>

---

# Lifetime Annotations

<div class="relative">
<div class="code-pointer" v-click="[1, 6]" style="top: 12px">▶</div>

````md magic-move {at: 6}
```rust
fn host_or_default(configured: &str, default: &str) -> &str {
    if configured.is_empty() {
        default
    } else {
        configured
    }
}
```
```rust
fn host_or_default<'a>(configured: &'a str, default: &'a str) -> &'a str {
    if configured.is_empty() {
        default
    } else {
        configured
    }
}
```
````

</div>

<div class="grid mt-4">
<div v-click="[1, 6]" class="small-code" style="grid-area: 1 / 1">

```text {1|4-5|7-9|10-13|all}{at:2}
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:56
  |
1 | fn host_or_default(configured: &str, default: &str) -> &str {
  |                                ----           ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
          but the signature does not say whether it is borrowed
          from `configured` or `default`
help: consider introducing a named lifetime parameter
  |
1 | fn host_or_default<'a>(configured: &'a str, default: &'a str) -> &'a str {
  |                   ++++              ++                ++          ++
```

</div>
<div v-click="6" style="grid-area: 1 / 1">

`'a` names a lifetime. The returned reference may only be used while both arguments are still valid.

In most functions the compiler infers lifetimes on its own. Annotations are only needed when it cannot tell which input a returned reference comes from.

<div v-click="7" class="mt-4">

`'static` is the lifetime of the entire program run. String literals such as `"localhost"` are `&'static str`. `thread::spawn` requires it (chapter 8).

</div>

</div>
</div>

---
class: dense
---

# Ownership Summary

<div class="flex flex-col gap-2 mt-3 compact">
  <div class="card" v-click>Each value has exactly one owner, usually a variable. When the owner goes out of scope, the value is dropped.</div>
  <div class="card" v-click>Assignment and passing move the value. Deep copies only happen with <code>clone()</code>. Simple types such as numbers are <code>Copy</code>.</div>
  <div class="card" v-click>References: any number of <code>&</code> for reading or one <code>&mut</code> for modifying. A reference never outlives its value.</div>
  <div class="card" v-click>Functions usually take borrowed types such as <code>&str</code>, <code>&Path</code> or <code>&[T]</code>.</div>
  <div class="card" v-click>Ownership and borrowing are checked at compile time, with no runtime cost.</div>
  <div class="card" v-click style="border-color: var(--mp-orange)"><span class="c-orange font-600">Typical fixes</span> when the compiler complains: pass <code>&</code> instead of moving, <code>clone()</code> if the cost is acceptable, use a reference before modifying the value, or own the data (<code>String</code> instead of <code>&str</code>).</div>
</div>
