---
theme: default
title: Rust - An Introduction
info: |
  Introduction to the Rust programming language for beginners (about 60 minutes).
author: Lukas Potthast
colorSchema: dark
transition: slide-left
mdc: true
lineNumbers: false
selectable: true
fonts:
  sans: Inter
  mono: JetBrains Mono
layout: cover
---

# Rust

<div class="text-3xl mt-4 font-600">An Introduction</div>

<div class="absolute bottom-12 left-20 muted">
  Lukas Potthast
</div>

<img src="/ferris.svg" alt="Ferris" class="ferris absolute right-10 top-1/2 -translate-y-1/2 w-80" />

<!--
- Short introduction, my own connection to Rust
- Goal: an overview of the language, not a programming course
- Questions at any time
-->

---

# Agenda

<div class="grid grid-cols-2 gap-x-12 gap-y-1 mt-8 text-lg">
<div>

1. Why Rust?
2. Tooling & Cargo
3. Syntax Basics
4. Ownership & Borrowing
5. Structs, Enums & Pattern Matching

</div>
<div>

6. Traits & Generics
7. Error Handling
8. Threads & Async
9. Ecosystem
10. Conclusion

</div>
</div>

<!--
Schedule (minutes, with the slide where each part starts and the time at which you should be there):

| Part | Minutes | Starts at slide | Clock |
| --- | --- | --- | --- |
| Intro | 1 | 1 | 0:00 |
| Why Rust | 4 | 3 | 0:01 |
| Tooling (Toolchain, Cargo) | 3 | 9 | 0:05 |
| Syntax basics (incl. tests) | 6 | 15 | 0:08 |
| Ownership | 12 | 22 | 0:14 |
| Structs, newtypes, enums, Option & Result | 8 | 33 | 0:26 |
| Traits | 5 | 43 | 0:34 |
| Error handling | 4 | 48 | 0:39 |
| Smart pointers, threads, futures & async | 8 | 54 | 0:43 |
| Ecosystem (crates.io, serde) | 3 | 64 | 0:51 |
| Conclusion (Order API, summary, AI assessment, use cases, resources) | 5 | 71 | 0:54 |
| Buffer for questions | 1 | 77 | 0:59 |

Slides that can be skipped when short on time are marked "Optional" in the notes, e.g. Cargo.toml, Semantic Versioning, Workspaces, Modules, Lifetimes, rootcause, Channels and most of the Ecosystem chapter. The plan assumes they are skipped or shown only briefly. If you are behind a checkpoint, skip the next optional slides.
-->

---
src: ./pages/01-why-rust.md
---

---
src: ./pages/02-tooling.md
---

---
src: ./pages/03-basics.md
---

---
src: ./pages/04-ownership.md
---

---
src: ./pages/05-data-types.md
---

---
src: ./pages/06-traits.md
---

---
src: ./pages/07-errors.md
---

---
src: ./pages/08-concurrency.md
---

---
src: ./pages/09-ecosystem.md
---

---
src: ./pages/10-conclusion.md
---
