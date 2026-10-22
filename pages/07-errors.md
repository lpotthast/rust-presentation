---
layout: section
---

<div class="chapter">Chapter 7</div>

# Error Handling

---
class: dense
---

# Kinds of Errors

Rust has no exceptions. It distinguishes two kinds of errors.

<div class="grid grid-cols-2 gap-8 mt-4 compact">
  <div class="card flex flex-col">
    <h3 class="c-green">Expected errors: <code>Result&lt;T, E&gt;</code></h3>
    <p>Missing file, network unreachable, invalid input. The caller decides how to handle them.</p>

<div class="mt-3">

```rust
let port: Result<u16, _> = "80a".parse();
// Err(ParseIntError { kind: InvalidDigit })
```

</div>

<p class="muted" style="margin-top: auto; padding-top: 0.5rem">Roughly: checked exception</p>
  </div>
  <div class="card flex flex-col" v-click="1">
    <h3 class="c-red">Programming errors: <code>panic!</code></h3>
    <p>Index out of bounds, violated invariant. The thread unwinds and stops. In <code>main</code>, this ends the program.</p>

<div class="mt-3">

```rust
let size = sizes[5]; // sizes has 3 elements
// panic: index out of bounds
```

</div>

<p class="muted" style="margin-top: auto; padding-top: 0.5rem">Roughly: unchecked exception, not meant to be caught</p>
  </div>
</div>

<div v-click="2" class="mt-6">

For indexing, the standard library offers both: `sizes[5]` panics, `sizes.get(5)` returns `None`. Libraries generally report errors as `Result` and avoid panics.

</div>

---
class: dense
---

# Handling a `Result`

The same input, four ways to handle the error.

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

<div class="relative">

```rust {all|3-6|7|8|9|8}
let input = "80a"; // e.g. read from a config file

let port: u16 = match input.parse() {
    Ok(port) => port,
    Err(_) => 8080,
};
let port: u16 = input.parse().unwrap_or(8080);
let port: u16 = input.parse().expect("port must be a number");
let port: u16 = input.parse()?;
```

<div class="code-pointer" v-click="5" style="top: 121px">▶</div>
</div>

<div v-click="5" class="mt-3">

```text
thread 'main' (38335095) panicked at src/main.rs:9:35:
port must be a number: ParseIntError { kind: InvalidDigit }
```

</div>

</div>
<div class="text-base">

<div v-click="1">

- `match`: any handling, here a default port

</div>
<div v-click="2">

- `unwrap_or`: the same as a one-liner

</div>
<div v-click="3">

- `expect`: panics with the message on `Err`. `unwrap()` does the same without a message.

</div>
<div v-click="4">

- `?`: returns the error to the caller (slide The `?` Operator)

</div>

</div>
</div>

---
class: dense
---

# The `Error` Trait

<div class="grid grid-cols-[2fr_3fr] gap-8">
<div>

```rust
// standard library, simplified
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&dyn Error> {
        None // the underlying cause
    }
}
```

<div v-click="1" class="text-base mt-3">

`Error: Debug + Display`: every error type must also implement `Debug` and `Display`. `source()` has a default implementation.

</div>

</div>
<div>

<div v-click="2">

```rust
#[derive(Debug)]
struct ReservedPort(u16);

impl fmt::Display for ReservedPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "port {} is reserved", self.0)
    }
}

impl Error for ReservedPort {}
```

</div>

<div v-click="3" class="text-base mt-3">

`io::Error`, `ParseIntError` and the other error types of the standard library implement `Error` as well. `Box<dyn Error>` can hold any of them (next slide).

</div>

</div>
</div>

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

`Box<dyn Error>`: any type that implements `Error` (previous slide). `Box` stores the value on the heap.

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

<div class="grid error-swap">
<div v-click-hide="3" style="grid-area: 1 / 1">

The function body is the same as on the previous slide. Only the error type changed.

</div>
<div v-click="3" style="grid-area: 1 / 1">

The third variant has no underlying error. Ports below 1024 are reserved.

</div>
</div>

`#[error(...)]` generates the `Display` and `Error` implementations, `#[from]` generates the `From` implementation that `?` uses.

<div v-click="6" class="mt-4">

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

<style>
.error-swap p { margin-bottom: 0; }
</style>

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
