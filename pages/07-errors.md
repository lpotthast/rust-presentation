---
layout: section
---

<div class="chapter">Chapter 7</div>

# Error Handling

<!--
- Passing errors on with ?, custom error types, and when to panic
-->

---
class: dense
---

# Kinds of Errors

<div class="grid grid-cols-2 gap-8 mt-4">
  <div class="card">
    <h3 class="c-green">Expected errors: <code>Result&lt;T, E&gt;</code></h3>
    <p>Missing file, network unreachable, invalid input. The caller decides how to handle them.</p>
    <p class="muted mt-1">Roughly: checked exception</p>
  </div>
  <div class="card">
    <h3 class="c-red">Programming errors: <code>panic!</code></h3>
    <p>Index out of bounds, violated invariant. The thread unwinds and stops. In <code>main</code>, this ends the program.</p>
    <p class="muted mt-1">Roughly: unchecked exception, not meant to be caught</p>
  </div>
</div>

<div v-click="1" class="mt-4">

A panic at runtime:

<div class="relative">
<div class="code-pointer" style="top: 30px">▶</div>

```rust
let sizes = vec!["S", "M", "L"];
let size = sizes[5]; // src/main.rs, line 3
```

</div>

<div class="small-code mt-2">

```text
thread 'main' (34883817) panicked at src/main.rs:3:21:
index out of bounds: the len is 3 but the index is 5
```

</div>
</div>

<div v-click="2" class="mt-4">
We know <code>Result</code> from chapter 5. This chapter covers passing errors on and custom error types.
</div>

<!--
- Compared to Java and C#: Result is roughly a checked exception (Java), the possible error is visible in the signature. A panic is roughly an unchecked exception (RuntimeException), but it is not meant to be caught. C# has no checked exceptions, there the error is not visible in the signature at all.
- Click 1: a real panic. Indexing out of bounds is a programming error, so Rust panics instead of returning a Result. The number in parentheses is the thread id. With RUST_BACKTRACE=1 the program also prints the call stack.
- sizes.get(5) returns an Option instead, for cases where a missing index is expected
- An unused Result causes a compiler warning (#[must_use])
-->

---
class: dense
---

# Using `Result`

```rust
use std::fs;

fn main() {
    match fs::read_to_string("port.txt") {
        Ok(content) => println!("Port: {content}"),
        Err(error) => eprintln!("Could not read file: {error}"),
    }
}
```

The value is only accessible after the error case has been handled.

<div v-click="1">

`unwrap` and `expect` take the value directly and cause a panic on error. They are suitable for tests and prototypes:

<div class="relative">
<div class="code-pointer" v-click="2" :style="{ top: ($clicks >= 3 ? 30 : 12) + 'px' }">▶</div>

```rust
let port: u16 = "80a".parse().unwrap();
let port: u16 = "80a".parse().expect("port must be a number");
```

</div>

</div>

<div class="grid mt-2">
<div v-click="[2, 3]" class="small-code" style="grid-area: 1 / 1">

```text
thread 'main' (35107165) panicked at src/main.rs:2:35:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

</div>
<div v-click="3" class="small-code" style="grid-area: 1 / 1">

```text
thread 'main' (35107172) panicked at src/main.rs:3:35:
port must be a number: ParseIntError { kind: InvalidDigit }
```

</div>
</div>

<!--
- Click 1: unwrap and expect on input that is not a number. Both panic.
- Click 2: the output of the unwrap line, a generic message.
- Click 3: the output of the expect line (as if the unwrap line were removed, otherwise the program stops at line 2). expect shows our own message, which makes the cause easier to find. The two lines are lines 2 and 3 of main.rs.
- Use unwrap/expect in production code only when an error is impossible
- Clippy can forbid unwrap via a lint
-->

---
class: dense
---

# The `?` Operator

````md magic-move
```rust
use std::error::Error;
use std::fs;

fn read_port(path: &str) -> Result<u16, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };
    let port: u16 = match content.trim().parse() {
        Ok(port) => port,
        Err(error) => return Err(error.into()),
    };
    Ok(port)
}
```
```rust {5-6|all}
use std::error::Error;
use std::fs;

fn read_port(path: &str) -> Result<u16, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    Ok(port)
}
```
````

<div class="muted text-base mt-2">

`Box<dyn Error>`: any error type. `dyn` as in chapter 6, `Box` stores the value on the heap (chapter 8).

</div>

<div class="grid grid-cols-2 gap-6 mt-4">
  <div class="card" v-click="2">
    <h3 class="c-orange">How it works</h3>
    <p>On <code>Ok</code>, the value is unwrapped. On <code>Err</code>, the function returns the error immediately and converts it with the standard trait <code>From</code>, exactly like <code>error.into()</code> in the long form. <code>main</code> can also return a <code>Result</code>, then <code>?</code> works there too.</p>
  </div>
  <div class="card" v-click="3">
    <h3 class="c-orange">Advantage</h3>
    <p>Code as compact as with exceptions, and every error source is marked by the <code>?</code> in the code.</p>
  </div>
</div>

<!--
- Start: read a port number from a file, with match. Two possible errors: the file cannot be read, the content is not a number. Each match returns early on Err.
- error.into() converts the concrete error (io::Error, ParseIntError) into Box<dyn Error>. into() and From are the standard conversion traits (Standard Traits slide, chapter 6).
- Click 1: the same function with ?, the two ? lines highlighted. Each ? replaces one of the match blocks.
- fn main() -> Result<(), Box<dyn Error>> lets ? propagate errors out of main, the program then prints the error and exits with a failure code.
- Box<dyn Error>: any error type, the caller no longer knows the concrete error. More precise errors with custom error types (next slide).
- ? also works with Option
- parse() converts text into the type given on the left (here u16)
-->

---
class: dense
---

# Custom Error Types

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

````md magic-move
```rust {all|4-11|13-17}
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io(#[from] std::io::Error),

    #[error("invalid port")]
    Parse(#[from] std::num::ParseIntError),
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    Ok(port)
}
```
```rust {12-13|19-21|all}
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io(#[from] std::io::Error),

    #[error("invalid port")]
    Parse(#[from] std::num::ParseIntError),

    #[error("port {0} is reserved")]
    Reserved(u16),
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    if port < 1024 {
        return Err(ConfigError::Reserved(port));
    }
    Ok(port)
}
```
````

</div>
<div class="text-base">

<div class="grid">
<div v-click-hide="3" style="grid-area: 1 / 1">

The function body is the same as on the previous slide. Only the error type changed.

</div>
<div v-click="3" style="grid-area: 1 / 1">

The third variant has no underlying error. Ports below 1024 are reserved.

</div>
</div>

`#[error(...)]` generates the `Display` implementation, `#[from]` generates the `From` implementation that `?` uses.

<div v-click="6" class="mt-4 small-code">

The caller can react to individual errors:

```rust
match read_port("port.txt") {
    Ok(port) => println!("port {port}"),
    Err(ConfigError::Io(_)) => println!("default 8080"),
    Err(error) => eprintln!("{error}"),
}
```

</div>

<div v-click="7" class="mt-4 muted">

Common practice: `thiserror` for libraries, `anyhow` for applications.

</div>

</div>
</div>

<!--
- Click 1 and 2: the error enum with one variant per error source, then read_port. Same body as on the previous slide.
- Click 3 to 5: a third error that does not come from a library: ports below 1024 are reserved
- Click 6: the payoff compared to Box<dyn Error>. The caller can match on the variants, e.g. fall back to a default port when the file is missing.
- The messages do not repeat the cause. The original error stays available as the source and is printed by anyhow as "Caused by: ...".
- thiserror and anyhow are both by David Tolnay
- anyhow: collect errors and print them with context, without custom error types
- Complete code: cargo run --example 07_thiserror
-->

---
class: dense
---

# Error Reports with `rootcause`

The same `read_port` as before. `ConfigError` from thiserror stays, rootcause adds the chain of causes.

<div class="grid grid-cols-[11fr_9fr] gap-8 mt-2">
<div>

````md magic-move
```rust
#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io(#[from] std::io::Error),
    #[error("invalid port")]
    Parse(#[from] std::num::ParseIntError),
    #[error("port {0} is reserved")]
    Reserved(u16),
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    if port < 1024 {
        return Err(ConfigError::Reserved(port));
    }
    Ok(port)
}
```
```rust {4,6|11|12-13|15|all}
#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io,
    #[error("invalid port")]
    Parse,
    #[error("port {0} is reserved")]
    Reserved(u16),
}

fn read_port(path: &str) -> Result<u16, Report<ConfigError>> {
    let content = fs::read_to_string(path).context(ConfigError::Io)?;
    let port: u16 = content.trim().parse().context(ConfigError::Parse)?;
    if port < 1024 {
        return Err(report!(ConfigError::Reserved(port)));
    }
    Ok(port)
}
```
````

</div>
<div>

<div v-click="6">

```rust
fn start_server(path: &str) -> Result<(), Report> {
    let port = read_port(path)
        .context("server could not start")
        .attach(format!("config file: {path}"))?;
    // ...
}
```

</div>

<div v-click="7" class="small-code mt-3">

```text
 ● server could not start
 ├ src/main.rs:28
 ├ config file: port.txt
 │
 ● cannot read file
 ├ src/main.rs:17
 │
 ● No such file or directory (os error 2)
 ╰ src/main.rs:17
```

</div>

<div v-click="8" class="text-base mt-3">

`Report<ConfigError>` is typed, the caller can match on it. `Report` is untyped, like anyhow.

</div>

</div>
</div>

<!--
- Optional
- Start: the code from the previous slide, written more compactly
- Click 1: the variants no longer store the cause. The cause stays in the report as its own node.
- Click 2: return type Report<ConfigError>. Click 3: .context(...) turns the io or parse error into a report and puts ConfigError on top. Click 4: report! creates a report for an error without a cause.
- Click 6: an application function with the untyped Report. .context(...) adds a node with a message, .attach(...) adds information for the log.
- Click 7: the printed report, with source location and attachment for each node. Output of cargo run --example 07_rootcause, path shortened.
- The caller matches with report.current_context(), which returns &ConfigError.
- Typed where the caller wants to react (the default port for ConfigError::Io, as on the previous slide, now via report.current_context()), untyped where errors are only passed on and logged. ? converts a typed report into an untyped one.
- thiserror still generates Display and Error for the enum
- rootcause is still at 0.x (0.13), so updates may contain breaking changes. Alternatives: anyhow, eyre, error-stack.
-->
