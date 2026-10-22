---
layout: section
---

<div class="chapter">Chapter 10</div>

# Conclusion

<!--
- Starts with a combined example that uses most chapters, then summary, assessment with AI assistants, use cases and resources
-->

---
class: dense
---

# Putting It Together: A Small Order API

<div class="grid grid-cols-2 gap-6">
<div>

```rust {all|1-7|9-21|none|none|none|all}{at:1}
#[derive(Serialize)]
#[serde(tag = "status")]
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

// stands in for a database query
fn find_order(id: u32) -> Option<OrderStatus> {
    match id {
        1 => Some(OrderStatus::Pending),
        2 => Some(OrderStatus::Shipped {
            tracking_id: String::from("DE123"),
        }),
        3 => Some(OrderStatus::Cancelled {
            reason: String::from("out of stock"),
        }),
        _ => None,
    }
}
```

</div>
<div>

```rust {all|none|none|1-7|4-5|9-14|all}{at:1}
async fn order_status(
    Path(id): Path<u32>,
) -> Result<Json<OrderStatus>, StatusCode> {
    let status = find_order(id)
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(status))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/orders/{id}", get(order_status));
    // bind listener, axum::serve(...)
}
```

<div class="grid mt-2">
  <div v-click="[1, 2]" class="muted text-base" style="grid-area: 1 / 1">Enum with data (chapter 5), JSON via <code>derive</code> (chapters 6 and 9)</div>
  <div v-click="[2, 3]" class="muted text-base" style="grid-area: 1 / 1"><code>Option</code> instead of <code>null</code> (chapter 5)</div>
  <div v-click="[3, 4]" class="muted text-base" style="grid-area: 1 / 1"><code>async</code> handler (chapter 8) with <code>Result</code> as return type (chapter 7)</div>
  <div v-click="[4, 5]" class="muted text-base" style="grid-area: 1 / 1"><code>ok_or</code> turns the <code>Option</code> into a <code>Result</code>, <code>?</code> returns 404 (chapter 7)</div>
  <div v-click="[5, 6]" class="muted text-base" style="grid-area: 1 / 1">Router with a path parameter (chapter 9)</div>
  <div v-click="6" class="small-code" style="grid-area: 1 / 1">

```text
$ curl localhost:3000/orders/2
{"status":"Shipped","tracking_id":"DE123"}
$ curl -i localhost:3000/orders/9
HTTP/1.1 404 Not Found
$ curl -i localhost:3000/orders/abc
HTTP/1.1 400 Bad Request
Invalid URL: Cannot parse `abc` to a `u32`
```

  </div>
</div>

</div>
</div>

<!--
- Goal: show how the chapters fit together. No need to explain every line. The caption under the code names the chapter for each step.
- Click 1: enum with data. serde generates the JSON, tag = "status" writes the variant into a field.
- Click 2: Option instead of null, match on the id. find_order stands in for a database query.
- Click 3: async handler with Result as the return type. axum reads the id from the path. Path(id): Path<u32> is a pattern in the parameter, it unpacks the value from the Path wrapper directly into id, like let (x, y) = point; for tuples.
- Click 4: ok_or turns the Option into a Result, ? returns 404 directly on None
- Click 5: router as on the axum slide, now with a path parameter
- Each step highlights only the lines it talks about, the other block is dimmed
- Click 6: output with curl. /orders/abc is rejected with 400 Bad Request before the handler runs, because id is a u32 (same idea as the u8 in the clap example). Response headers are left out on the slide.
- Ownership is involved everywhere without having to write it out: status is moved into Json and belongs to axum afterwards.
- The use lines and the listener are left out for space. Complete and runnable: cargo run --example 10_order_api
-->

---

# Summary

The three goals from the start of the talk, with the chapters that showed them.

<div class="grid grid-cols-3 gap-6 mt-6">
  <div class="card" v-click>
    <h3 class="c-orange">Performance</h3>
    <p>Native code without a garbage collector (chapter 1). Iterator chains compile to the same code as a hand-written loop (chapter 6).</p>
  </div>
  <div class="card" v-click>
    <h3 class="c-green">Reliability</h3>
    <p>Use after move and dangling references do not compile (chapter 4), neither does <code>Rc</code> shared across threads (chapter 8). A missing <code>match</code> arm is reported, errors are part of the return type (chapters 5 and 7).</p>
  </div>
  <div class="card" v-click>
    <h3 class="c-blue">Productivity</h3>
    <p>Cargo builds, tests and manages dependencies (chapter 2). Compiler errors show the cause and a fix (chapters 4 and 5). crates.io offers libraries for most tasks (chapter 9).</p>
  </div>
</div>

<!--
- Same three headings as on the slide What Is Rust? at the beginning. Each point names a moment from the talk.
- Afterwards: assessment with AI assistants, use cases and where to start
-->

---
class: dense
---

# Pros & Cons with AI Assistants

<div class="grid grid-cols-2 gap-8 mt-4">
  <div class="card">
    <h3 class="c-green">Pros</h3>
    <table class="eval"><tbody>
      <tr><td>Compiler errors with fixes</td><td><span v-click="1" class="tag tag-green">AI applies the fixes</span></td></tr>
      <tr><td>Signatures show <code>Result</code>, <code>&amp;mut</code></td><td><span v-click="1" class="tag tag-green">AI sees the contract</span></td></tr>
      <tr><td>Tooling: <code>cargo check</code>, <code>clippy</code></td><td><span v-click="1" class="tag tag-green">AI runs them too</span></td></tr>
    </tbody></table>
  </div>
  <div class="card" v-click="2">
    <h3 class="c-red">Cons</h3>
    <table class="eval"><tbody>
      <tr><td>Learning ownership</td><td><span v-click="3" class="tag tag-green">AI explains the errors</span></td></tr>
      <tr><td>Complex signatures (rare)</td><td><span v-click="3" class="tag tag-green">AI writes them</span></td></tr>
      <tr><td>Async is more complex</td><td><span v-click="3" class="tag tag-yellow">a bit easier with AI</span></td></tr>
      <tr><td>Long compile times</td><td><span v-click="3" class="tag tag-red">matters more for agents</span></td></tr>
      <tr><td>Young areas, e.g. GUI</td><td><span v-click="3" class="tag tag-muted">unchanged</span></td></tr>
    </tbody></table>
  </div>
</div>

<div v-click="3" class="muted text-sm mt-3">
<span class="c-green">green</span>: AI helps, <span class="c-yellow">yellow</span>: partly, grey: no effect, <span class="c-red">red</span>: weighs more with AI
</div>

<div v-click="4" class="mt-3">
Understanding ownership is still needed to review generated code. The Rust compiler does not find logic errors either.
</div>

<!--
- Click 1: the pros that also help an AI assistant. Performance and the rest of the Summary are not repeated here, they do not depend on AI.
- Tag colors mean the same in both columns, the legend appears with click 3
- Clicks 2 and 3: the cons, and how much AI assistants reduce them. Compile times, learning curve and type signatures are the Costs rows from the Comparison in chapter 1. Compile times are longer than Go or Java.
- My own assessment from practice, not a study
- In languages with weak guarantees, faulty generated code compiles and only fails at runtime. In Rust the compiler rejects many of these errors, and the assistant fixes them in the next iteration.
- Learning: assistants explain compiler errors and suggest fixes. The assistant writes and explains type signatures and lifetimes.
- Async: error messages stay long, assistants help with reading them
- Compile times matter even more for AI agents because they rebuild often. cargo check is much faster than a full build.
- Learning ownership: in my experience a few weeks of regular use
- Google (Lars Bergstrom, Rust Nation UK 2024): Rust teams are about as productive as Go teams and roughly twice as productive as C++ teams
-->

---
class: dense
---

# Use Cases

<div class="grid grid-cols-2 gap-8 mt-4">
  <div class="card">
    <h3 class="c-green">Good fit</h3>
    <ul>
      <li>Services with performance or latency requirements</li>
      <li>Command-line tools and system utilities</li>
      <li>Performance-critical modules for Python or Node.js</li>
      <li>Security-critical components: parsers, network code, cryptography</li>
      <li>Embedded and IoT</li>
      <li>WebAssembly</li>
    </ul>
  </div>
  <div class="card" v-click="1">
    <h3 class="c-yellow">Less suitable</h3>
    <ul>
      <li>Short-lived scripts where a build step gets in the way</li>
      <li>Areas where another language's ecosystem is decisive (e.g. ML training in Python)</li>
      <li>CRUD applications without performance requirements, where a garbage-collected language is sufficient</li>
    </ul>
  </div>
</div>

<div v-click="2" class="card mt-6" style="border-color: var(--mp-orange)">
  <h3 class="c-orange">Where to start</h3>
  <p>An internal command-line tool or a small, well-scoped service.</p>
</div>

<!--
- Relate to our own projects
- The order API in this chapter is small to fit on a slide, it is not a recommendation to write simple CRUD services in Rust
- Prototypes: with AI assistants the writing effort drops. Learning Rust is still needed but goes faster (see slide Pros & Cons).
- Click 2: a first project with little risk. A CLI tool needs no deployment, a small service can be replaced if needed.
-->

---
class: dense
---

# Resources

<div class="grid grid-cols-2 gap-3 mt-4 compact">
  <div class="card" style="border-color: var(--mp-green)"><h3 class="c-green">Start here: The Rust Book</h3><p><a href="https://doc.rust-lang.org/book/">doc.rust-lang.org/book</a>: the official book, free. German translation: <a href="https://rust-lang-de.github.io/rustbook-de/">rust-lang-de.github.io/rustbook-de</a></p></div>
  <div class="card" style="border-color: var(--mp-green)"><h3 class="c-green">Start here: Rustlings</h3><p><a href="https://rustlings.rust-lang.org">rustlings.rust-lang.org</a>: small exercises in the terminal, alongside the book</p></div>
  <div class="card"><h3 class="c-orange">Comprehensive Rust</h3><p><a href="https://google.github.io/comprehensive-rust/">google.github.io/comprehensive-rust</a>: course by Google, also available in German</p></div>
  <div class="card"><h3 class="c-orange">Rust by Example</h3><p><a href="https://doc.rust-lang.org/rust-by-example/">doc.rust-lang.org/rust-by-example</a>: language features explained through examples</p></div>
  <div class="card"><h3 class="c-orange">Rust Playground</h3><p><a href="https://play.rust-lang.org">play.rust-lang.org</a>: run Rust in the browser</p></div>
  <div class="card"><h3 class="c-orange">blessed.rs</h3><p><a href="https://blessed.rs/crates">blessed.rs/crates</a>: recommended crates for common tasks</p></div>
</div>

<div class="mt-4">

Slides and all examples from this talk: <a href="https://github.com/lpotthast/rust-presentation">github.com/lpotthast/rust-presentation</a>. The examples are in `code-examples` and runnable, e.g. `cargo run --example 05_newtype`.

</div>

<!--
- Recommendation for getting started: the Rust Book, with Rustlings alongside. The other resources are for later or for reference.
- Comprehensive Rust: developed by the Android team at Google, a 4-day core course plus deep dives (Android, Chromium, bare metal, concurrency)
- code-examples contains every example from the slides, plus the intentionally failing ones in compile-errors/
- blessed.rs answers "which crate should I use for X?"
- More exercises with mentoring: exercism.org/tracks/rust
-->

---
layout: center
class: text-center closing
---

<img src="/ferris.svg" alt="Ferris" class="ferris w-56 mx-auto mb-4" />

# Questions?

<div class="muted text-base mt-8">
<a href="https://github.com/lpotthast/rust-presentation">github.com/lpotthast/rust-presentation</a><br>
<a href="https://doc.rust-lang.org/book/">doc.rust-lang.org/book</a> · <a href="https://rustlings.rust-lang.org">rustlings.rust-lang.org</a>
</div>

<!--
- First link: the repository with the slides and code-examples. Below it: the two starting points from the Resources slide.
- Show the Resources slide again if needed
-->
