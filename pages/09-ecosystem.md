---
layout: section
---

<div class="chapter">Chapter 9</div>

# Ecosystem

<!--
- Libraries for typical tasks in real projects
- Order: overview, then the optional slides, then serde and axum. axum leads directly into the combined example in chapter 10.
-->

---
class: dense
---

# crates.io

More than 350,000 crates, each with API documentation on docs.rs. Crates in <strong>yellow</strong> appear in this talk.

<div class="grid grid-cols-4 gap-2 mt-3 compact">
  <div class="card"><h3 class="c-orange">Serialization</h3><p><strong>serde</strong>, <strong>serde_json</strong>, toml</p></div>
  <div class="card"><h3 class="c-orange">Async</h3><p><strong>tokio</strong>, futures</p></div>
  <div class="card"><h3 class="c-orange">Web servers</h3><p><strong>axum</strong>, actix-web</p></div>
  <div class="card"><h3 class="c-orange">HTTP clients</h3><p><strong>reqwest</strong>, hyper</p></div>
  <div class="card"><h3 class="c-orange">Errors</h3><p><strong>thiserror</strong>, <strong>anyhow</strong>, <strong>rootcause</strong></p></div>
  <div class="card"><h3 class="c-orange">Command line</h3><p><strong>clap</strong>, indicatif, ratatui</p></div>
  <div class="card"><h3 class="c-orange">Testing</h3><p><strong>assertr</strong>, proptest, insta</p></div>
  <div class="card"><h3 class="c-orange">Logging</h3><p>tracing, log</p></div>
  <div class="card"><h3 class="c-orange">Databases</h3><p>sqlx, diesel, sea-orm</p></div>
  <div class="card"><h3 class="c-orange">Parallelism</h3><p>rayon, crossbeam</p></div>
  <div class="card"><h3 class="c-orange">Desktop & GUI</h3><p>tauri, egui, iced</p></div>
  <div class="card"><h3 class="c-orange">Benchmarks</h3><p>criterion, divan</p></div>
  <div class="card"><h3 class="c-orange">Embedded</h3><p>embassy, probe-rs</p></div>
  <div class="card"><h3 class="c-orange">WebAssembly</h3><p>wasm-bindgen, leptos</p></div>
  <div class="card"><h3 class="c-orange">Python</h3><p>pyo3, maturin</p></div>
  <div class="card"><h3 class="c-orange">C and C++</h3><p>bindgen, cxx</p></div>
</div>

<!--
- Rows from top to bottom: web backend, every application, data and desktop, other platforms
- The yellow crates were used in the examples of this talk. Some of them only on optional slides: reqwest (Cargo.toml), rootcause (chapter 7), clap (this chapter).
- docs.rs generates the API documentation of every published crate in a uniform format
- blessed.rs: curated list of recommended crates (also on the Resources slide)
- Games: bevy
-->

---
class: dense
---

# Well-Known Tools Written in Rust

<div class="grid grid-cols-3 gap-4 mt-4">
  <div class="card">
    <h3 class="c-yellow">For Python</h3>
    <p><strong>uv</strong>: package manager<br><strong>ruff</strong>: linter and formatter<br><strong>Polars</strong>: DataFrame library</p>
  </div>
  <div class="card">
    <h3 class="c-yellow">For JavaScript / TypeScript</h3>
    <p><strong>Deno</strong>: runtime<br><strong>SWC</strong>, <strong>Turbopack</strong>, <strong>Rolldown</strong>: build tools</p>
  </div>
  <div class="card">
    <h3 class="c-yellow">For everyone</h3>
    <p><strong>ripgrep</strong>: search (inside VS Code)<br><strong>Zed</strong>: editor<br><strong>fd</strong>, <strong>bat</strong>: find and cat</p>
  </div>
</div>

<div class="mt-6">
Rust often replaces the performance-critical part of tools for other languages.
</div>

<div class="muted text-base mt-2">
Example: uv installs packages 10 to 100 times faster than pip (vendor figure).
</div>

<!--
- Optional
- ruff: also 10 to 100 times faster than Flake8 according to the vendor
- Polars: alternative to pandas
- VS Code runs ripgrep for "Find in Files", so most of the audience already uses Rust code daily
-->

---
class: dense
---

# Command-Line Tools with clap

<div class="grid grid-cols-2 gap-8">
<div>

```rust
use clap::Parser;

/// Reorders a product from the supplier
#[derive(Parser)]
struct Args {
    /// Product number, e.g. BANANA-01
    sku: String,

    /// Number of boxes to order
    #[arg(short, long, default_value_t = 1)]
    quantity: u8,
}

fn main() {
    let args = Args::parse();
    println!("Ordering {} x {}", args.quantity, args.sku);
}
```

</div>
<div>

<div v-click="1" class="small-code">

```text
$ reorder --help
Reorders a product from the supplier

Usage: reorder [OPTIONS] <SKU>

Arguments:
  <SKU>  Product number, e.g. BANANA-01

Options:
  -q, --quantity <QUANTITY>  Number of boxes to order
                             [default: 1]
  -h, --help                 Print help
```

</div>

<div v-click="2" class="small-code mt-3">

```text
$ reorder BANANA-01 --quantity 20
Ordering 20 x BANANA-01

$ reorder BANANA-01 --quantity 300
error: invalid value '300' for '--quantity <QUANTITY>':
       300 is not in 0..=255
```

</div>

<div v-click="2" class="muted text-base mt-2">
Help text, validation and error messages are generated from doc comments and types.
</div>

</div>
</div>

<!--
- Optional
- A small internal tool: reorder a product from the supplier
- The result is a single binary without runtime dependencies
- /// are doc comments, clap uses them as help text
- Click 2: quantity is a u8, so clap rejects 300 without any code for it
- Output of code-examples/examples/09_clap.rs, built as a binary named reorder (with cargo run the usage line shows the example name instead). The help line for --quantity and the error line are wrapped for the slide.
-->

---
class: dense
---

# Macros & `unsafe`

<div class="grid grid-cols-2 gap-8">
<div>

**Macros**: code generation at compile time

```rust
macro_rules! check {
    ($cond:expr) => {
        if !$cond {
            println!("check failed: {}", stringify!($cond));
        }
    };
}

let port = 80;
check!(port >= 1024); // check failed: port >= 1024
```

<div class="muted text-base">

A function `check(cond: bool)` would only receive `false`, not the text `port >= 1024`.

`println!` and `vec!` are macros too. Procedural macros: e.g. `#[derive(...)]` and `#[tokio::main]`.

</div>

</div>
<div v-click>

**unsafe**: operations whose safety the compiler cannot check

```rust
unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

// safe API around the C function
fn c_abs(x: i32) -> Option<i32> {
    if x == i32::MIN {
        return None; // undefined behavior in C
    }
    Some(unsafe { abs(x) })
}
```

<div class="muted text-base">
The compiler cannot check C code. The safe wrapper handles the one input for which <code>abs</code> is undefined. Callers of <code>c_abs</code> need no <code>unsafe</code>.
</div>

</div>
</div>

<!--
- Optional
- A macro receives the code itself, so stringify! can turn the condition into text
- #[derive(...)] is a procedural macro: the serde slide uses it to generate the JSON code
- abs(i32::MIN) has no valid result in an i32 and is undefined behavior in C. That is exactly the kind of precondition the compiler cannot check.
- unsafe allows e.g. dereferencing raw pointers and calling C code (FFI). It does not disable the borrow checker, it only allows additional operations.
- The standard library uses unsafe internally (e.g. Vec), the public API is safe
- For memory bugs, only the unsafe blocks need to be checked
- Makes it possible to introduce Rust step by step into existing C and C++ projects. C/C++ interop: bindgen, cbindgen, cxx
-->

---
class: dense
---

# serde

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|3-9|12-13|14,16|all}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    #[serde(default)]
    admin: bool,
}

fn main() {
    let json = r#"{ "id": 1, "name": "Kevin" }"#;
    let user: User = serde_json::from_str(json).unwrap();
    println!("{user:?}");

    println!("{}", serde_json::to_string_pretty(&user).unwrap());
}
```

</div>
<div v-click="4">

```text
User { id: 1, name: "Kevin", admin: false }
{
  "id": 1,
  "name": "Kevin",
  "admin": false
}
```

<div class="muted text-base mt-3">

`admin` is missing in the input and set to `false` by `#[serde(default)]`.

The code is generated at compile time via `derive`, without reflection at runtime as in Jackson (Java). Supports JSON, TOML and many other formats.

</div>

</div>
</div>

<!--
- One of the most widely used crates
- Click 1: the struct with derive, Click 2: parsing JSON into a User, Click 3: output as Debug and as JSON, Click 4: the real output
- unwrap() panics if the JSON is invalid (chapter 7). In real code: return a Result and use ?.
- Missing required fields or wrong types result in an Err
- r#"..."# is a raw string, quotes inside it need no escaping
- User matches the struct from chapter 5, extended with an id
- Output of cargo run --example 09_serde
-->

---
class: dense
---

# Web Server with axum

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|12-14|18|19-20|all}
use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    admin: bool,
}

async fn current_user() -> impl IntoResponse {
    Json(User { id: 1, name: String::from("Kevin"), admin: false })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/me", get(current_user));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

</div>
<div v-click="4">

```text
$ curl localhost:3000/me
{"id":1,"name":"Kevin","admin":false}
```

<div class="muted text-base mt-3">

`impl IntoResponse`: any type that axum can turn into an HTTP response. `Json(...)` sends the value as JSON with the matching content type.

</div>

</div>
</div>

<!--
- axum is developed by the Tokio team and is the most downloaded web framework on crates.io
- Combination of tokio, serde and axum. The User has the same fields as the User on the serde slide, only Serialize is needed here.
- Click 1: an async handler. impl IntoResponse in the return type means "some type that implements IntoResponse", as with impl Future on the Futures slide (chapter 8). Json(...) is such a type and sends the value as JSON.
- Click 2: the route /me points to the handler. Click 3: start the server on port 3000. Click 4: the response.
- Other types that implement IntoResponse: String, StatusCode, tuples like (StatusCode::CREATED, Json(...)), and Result of such types. The combined example right after this chapter writes the concrete type Result<Json<...>, StatusCode>, because it returns two different types (Ok and Err).
- Optional live demo: cd code-examples && cargo run --example 09_axum
-->
