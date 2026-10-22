---
layout: section
---

<div class="chapter">Chapter 2</div>

# Tooling & Cargo

---
class: dense
---

# Toolchain

<div class="grid grid-cols-2 gap-10 mt-4">
<div>

Installation and updates via **rustup**:

```bash
curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs | sh
rustup update   # update to the latest version
```

<div v-click="5" class="mt-6">

**Releases**

- New stable version every 6 weeks
- Channels: `stable`, `beta`, `nightly`
- **Editions** 2015, 2018, 2021, 2024 (every 3 years) bundle opt-in language changes, set per project in `Cargo.toml`

</div>
</div>
<div class="flex flex-col gap-2 compact">
  <div class="card" v-click="1"><h3 class="c-orange">rustc</h3><p>Compiler with an LLVM backend</p></div>
  <div class="card" v-click="2"><h3 class="c-orange">cargo</h3><p>Build system, package manager, test runner</p></div>
  <div class="card" v-click="3"><h3 class="c-orange">rustfmt & clippy</h3><p>Formatter and linter with over 800 lints</p></div>
  <div class="card" v-click="4"><h3 class="c-orange">rust-analyzer</h3><p>Language server for VS Code, Zed, Vim and others. Not installed by rustup by default.</p></div>
</div>
</div>

---
class: dense
---

# Cargo

<div class="grid grid-cols-2 gap-10">
<div>

```bash {all|1-5|6-8|9-10|all}
cargo new hello        # create a new project
cd hello
cargo run              # build and run
cargo check            # type-check only, fast
cargo build --release  # optimized build
cargo test             # run all tests
cargo fmt              # format code
cargo clippy           # run the linter
cargo doc --open       # build and open docs
cargo add serde        # add a dependency
```

</div>
<div>

Project structure (Cargo conventions):

````md magic-move {at: 4}
```bash
hello/
├── Cargo.toml     # project manifest
├── Cargo.lock     # exact versions
└── src/
    └── main.rs    # entry point
```
```bash
hello/
├── Cargo.toml     # project manifest
├── Cargo.lock     # exact versions
├── src/
│   ├── main.rs    # entry point (binary)
│   └── lib.rs     # library code
├── tests/         # integration tests
└── benches/       # benchmarks
```
````

</div>
</div>

<div v-click="5" class="mt-4">

Rust packages are called **crates**. Comparable to:

<div class="tight-table text-sm">

| Rust         | Java (Maven)    | JavaScript (npm)    | Python           |
| ------------ | --------------- | ------------------- | ---------------- |
| crate        | artifact        | package             | package          |
| `Cargo.toml` | `pom.xml`       | `package.json`      | `pyproject.toml` |
| `Cargo.lock` | (none)          | `package-lock.json` | e.g. `uv.lock`   |
| crates.io    | Maven Central   | npm registry        | PyPI             |

</div>
</div>

---
class: dense
---

# Cargo.toml

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```toml {all|1-4|6-7|8|9-12|14-15|17-19|all}
[package]
name = "my-service"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
reqwest = { version = "0.13.5", default-features = false, features = [
    "rustls",
    "json",
] }

[dev-dependencies]
assertr = "0.9"       # tests only

[profile.release]
lto = true            # link-time optimization
codegen-units = 1
```

</div>
<div class="text-base">

<div v-click="1" :class="{ 'opacity-40': $clicks > 1 && $clicks < 7 }">

- `edition = "2024"` selects the language edition (Toolchain slide)

</div>
<div v-click="2" :class="{ 'opacity-40': $clicks > 2 && $clicks < 7 }">

- `"1"` means `>= 1.0.0, < 2.0.0`

</div>
<div v-click="3" :class="{ 'opacity-40': $clicks > 3 && $clicks < 7 }">

- **Features** enable optional parts of a crate

</div>
<div v-click="4" :class="{ 'opacity-40': $clicks > 4 && $clicks < 7 }">

- `"0.13.5"` means `>= 0.13.5, < 0.14.0`. For 0.x versions, the second number counts as incompatible (Semantic Versioning, next slide).
- `default-features = false` disables the crate's default features

</div>
<div v-click="5" :class="{ 'opacity-40': $clicks > 5 && $clicks < 7 }">

- `dev-dependencies` are only built for tests and examples

</div>
<div v-click="6">

- Profiles control compiler options, here more optimization for release builds

</div>

</div>
</div>

---
class: dense
---

# Semantic Versioning

<div class="grid grid-cols-2 gap-10 mt-4">
<div>

<div class="semver-head">MAJOR.MINOR.PATCH</div>
<div class="muted text-sm mb-2">e.g. <code>1.38.1</code>: MAJOR 1, MINOR 38, PATCH 1</div>

<div class="semver-steps">
  <div class="semver-step" style="margin-left: 1.2rem" v-click><span class="semver-arrow">↳</span><div class="card"><p>incompatible API changes</p></div></div>
  <div class="semver-step" style="margin-left: 5.9rem" v-click><span class="semver-arrow">↳</span><div class="card"><p>new functionality, backward compatible</p></div></div>
  <div class="semver-step" style="margin-left: 10.6rem" v-click><span class="semver-arrow">↳</span><div class="card"><p>compatible bug fixes</p></div></div>
</div>

<div class="compact mt-6" v-click><div class="card"><p><span class="c-orange font-600">Before version 1.0.0</span> the first non-zero number counts as MAJOR. Examples of incompatible updates:</p>
<p class="mt-2"><code>0.4</code> → <code>0.5</code><br><code>0.0.3</code> → <code>0.0.4</code></p></div></div>

</div>
<div>

<div v-click>

| Requirement         | Allowed versions      | Meaning |
| ------------------- | --------------------- | --------- |
| `"1.38.1"`          | `>= 1.38.1, < 2.0.0`  | MINOR and PATCH updates |
| `"~1.38.1"`         | `>= 1.38.1, < 1.39.0` | PATCH updates |
| `"=1.38.1"`         | exactly `1.38.1`      | no updates |

</div>

<div v-click class="mt-4 text-base">

`~` is used when even new MINOR versions cause problems.

`=` pins a version, e.g. to avoid a bug in a newer version.

</div>

</div>
</div>

---
class: dense
---

# Workspaces

Multiple crates in one repository, built and tested together.

<div class="grid grid-cols-2 gap-8 mt-2">
<div>

```bash
shop/
├── Cargo.toml        # workspace root
├── Cargo.lock        # shared by all crates
├── target/           # shared build output
└── crates/
    ├── shop-core/    # library: domain logic
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── shop-api/     # binary: web server
        ├── Cargo.toml
        └── src/main.rs
```

<div v-click="1">

```toml
# shop/Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

</div>

</div>
<div>

<div v-click="2">

```toml
# shop/crates/shop-api/Cargo.toml
[package]
name = "shop-api"
version = "0.1.0"
edition = "2024"

[dependencies]
shop-core = { path = "../shop-core" }
serde = { workspace = true }
```

</div>

<div v-click="3">

```bash
cargo build              # builds all crates
cargo test -p shop-core  # only one crate
```

</div>

</div>
</div>
