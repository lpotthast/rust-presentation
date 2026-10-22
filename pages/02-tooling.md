---
layout: section
---

<div class="chapter">Chapter 2</div>

# Tooling & Cargo

<!--
- How to create and build a project, before we get to the language itself
-->

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

<!--
- One build system and one formatter for all projects, no choice to make like in C++ (CMake, Meson, Bazel ...)
- Editions: new compiler versions still compile old code as long as the crate stays on its edition. Switching to a new edition can break code, e.g. async became a keyword in edition 2018, so let async = 1; no longer compiles there. cargo fix --edition migrates most of it automatically.
- Clicks 1 to 4: what rustup installs. Click 5: how releases and editions work.
- A crate on edition 2015 can use a crate on edition 2024 without problems, and vice versa
- Windows: rustup-init.exe from rustup.rs or winget install Rustlang.Rustup. It needs the Visual Studio C++ Build Tools for linking, the installer offers to install them.
- In its default profile, rustup installs rustc, cargo, rustfmt and clippy. rust-analyzer comes separately: via the editor extension (e.g. VS Code) or with rustup component add rust-analyzer. RustRover has its own analysis engine.
-->

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

<!--
- Comparable to npm or Maven
- Click 1: create, build and run. Click 2: code quality (tests, formatting, linter). Click 3: documentation and dependencies. Click 4: all commands again, and the project structure gets more conventions. Click 5: the term crate and the comparison with other ecosystems.
- Maven has no lock file by default, versions are fixed in the pom.xml. Python: the lock file depends on the tool (uv.lock, poetry.lock).
- cargo check only type-checks without generating code. Much faster than cargo build, ideal while writing code.
- Packages are called crates, the central registry is crates.io
- Cargo.lock is created on the first build, cargo new only creates Cargo.toml and src/main.rs
- First state: after cargo new and the first build. Click 4: more conventions. Cargo detects these files and directories automatically, without configuration.
- Optional live demo: cargo new hello && cd hello && cargo run
-->

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

<!--
- Optional
- Each click highlights a section on the left and shows its explanation on the right. Earlier explanations are dimmed, at the last click everything is bright again.
- Cargo.lock contains the exact versions, which makes builds reproducible
- anyhow: simplest form, just the version
- serde: feature "derive" in addition to the default features, needed for #[derive(Serialize, Deserialize)]
- reqwest (HTTP client): default features turned off (e.g. HTTP/2, charset detection, system proxy), only TLS via rustls and JSON support enabled. Saves dependencies and compile time.
- assertr: library for readable assertions in tests, e.g. assert_that!(port).is_greater_than(1023). Only built for tests.
-->

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

<!--
- Optional
- Table: the same version three times, only the operator changes. Without an operator (the default, same as ^), Cargo allows all updates up to the next incompatible version.
- Before 1.0.0: the SemVer specification allows any change there. Cargo uses the rule "the first non-zero number is incompatible": in 0.4.2 the 4 is the incompatible position, an update to 0.4.3 may contain new functionality or bug fixes. With 0.0.x every new version counts as incompatible.
- Many popular crates are still at 0.x, e.g. rand 0.8 → 0.9 or axum 0.7 → 0.8 were real breaking changes.
- A requirement without an operator is the same as ^ (caret). This is the normal case, ~ and = are rarely needed.
- Avoid = in libraries: it prevents Cargo from finding a common version for all dependencies.
- The examples without an operator ("1", "0.13.5") are known from the previous slide.
- Cargo.lock records the versions actually used. cargo update updates within the allowed ranges.
- Different incompatible versions of the same crate can appear in the dependency tree at the same time
- The rules assume that crate authors follow SemVer. cargo-semver-checks verifies this automatically before a release.
-->

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

<!--
- Optional
- Typical use: splitting larger projects into a library and an application, or several services with shared code
- All crates share Cargo.lock and target/. Shared dependencies are built only once, all crates use the same versions.
- Click 1: the root Cargo.toml only contains [workspace]. members lists the crates, here via a pattern.
- [workspace.dependencies] defines versions in one place. The crates take them over with workspace = true.
- Click 2: dependency between crates in the workspace via path. Publishing to crates.io additionally requires a version.
- resolver = "3" is the current dependency resolver for edition 2024. A workspace root without [package] has no edition, so the resolver must be set explicitly. Otherwise Cargo falls back to resolver 1 and warns.
- Shorter compile times: after a change, only the affected crates are rebuilt
-->
