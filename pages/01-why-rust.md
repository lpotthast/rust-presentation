---
layout: section
---

<div class="chapter">Chapter 1</div>

# Why Rust?

---
class: dense
---

# What Is Rust?

<div class="text-2xl mt-6 font-600">
“A language empowering everyone<br>to build reliable and efficient software.”
</div>

<div class="muted mt-2">rust-lang.org</div>

<div class="grid grid-cols-3 gap-6 mt-8">
  <div class="card" v-click>
    <h3 class="c-orange">Performance</h3>
    <p>Fast and memory-efficient. With no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and integrate easily with other languages.</p>
  </div>
  <div class="card" v-click>
    <h3 class="c-green">Reliability</h3>
    <p>The type system and ownership model guarantee memory safety and thread safety. Many classes of bugs are eliminated at compile time.</p>
  </div>
  <div class="card" v-click>
    <h3 class="c-blue">Productivity</h3>
    <p>Great documentation, helpful compiler error messages, an integrated package manager and build tool, editor support and an auto-formatter.</p>
  </div>
</div>

<!--
- Quote: main slogan on rust-lang.org. The three cards are shortened versions of the three sections below it: Performance, Reliability, Productivity.
- rust-lang.org says "no runtime". This means no runtime like the JVM or .NET. For async you still need a runtime later, as a library (Tokio, chapter 8).
- Thread safety here means: data races are ruled out at compile time (chapter 8)
- Rust is a compiled language, stable since 2015 (version 1.0), used from microcontrollers to web backends
-->

---

# History

<div class="grid grid-cols-[1fr_auto] gap-10 mt-6">
<div class="timeline self-start">
  <div class="item" v-click="1"><span class="year">2006</span> Side project of Mozilla employee Graydon Hoare</div>
  <div class="item" v-click="1"><span class="year">2009</span> Mozilla starts funding the project (later Mozilla Research)</div>
  <div class="item" v-click="1"><span class="year">2015</span> Rust 1.0, backward compatible since then</div>
  <div class="item" v-click="2"><span class="year">2019</span> Stabilization of <code>async</code> / <code>await</code></div>
  <div class="item" v-click="2"><span class="year">2021</span> Independent Rust Foundation is founded</div>
  <div class="item" v-click="2"><span class="year">2022</span> First Rust support in the Linux kernel (6.1)</div>
  <div class="item" v-click="2"><span class="year">2025</span> Rust in the Linux kernel is no longer experimental</div>
</div>
<div class="card w-64 self-start" v-click="3">
  <h3 class="c-yellow">Origin</h3>
  <p>According to Graydon Hoare, the project started with a broken elevator in a Vancouver apartment building. Its software had crashed, and Hoare had to walk up to the 21st floor.</p>
</div>
</div>

<!--
- Click 1: origins until 1.0. Click 2: since 1.0. Click 3: the elevator story.
- The crashed elevator software was most likely a memory bug, which leads to the next slide.
- Hoare was employed at Mozilla but initially developed Rust as a side project, outside of work. The project was not public until 2010.
- In 2009 Mozilla started funding it, from 2010 Rust was an official Mozilla Research project. Public announcement in 2010.
- Source for the elevator story: MIT Technology Review, February 2023
- Name: according to Hoare, partly after rust fungi, which Hoare called "over-engineered for survival"
- Mozilla wanted a safer browser engine that uses multiple CPU cores (project Servo, from 2012). Parts of it, e.g. the CSS engine, shipped in Firefox Quantum in 2017.
- First Rust code in Firefox: 2016 (MP4 parser in Firefox 48)
- 2020 layoffs at Mozilla, 2021 founding of the independent Rust Foundation. Founding members: AWS, Google, Huawei, Microsoft, Mozilla.
- Kernel: the "Rust experiment" was declared over at the Maintainers Summit in December 2025. Some parts (e.g. certain architectures) are still considered experimental.
- Release cycle: a new version every 6 weeks
-->

---
class: dense
---

# Memory Bugs in C and C++

<div class="grid grid-cols-2 gap-10 mt-6">
<div>

Typical bug classes in C and C++:

<v-clicks>

- **Use-after-free:** accessing memory that was already freed
- **Buffer overflow:** writing past the end of an array
- **Double free:** memory is freed twice
- **Null pointer:** access through a pointer without a valid target <span class="tag tag-yellow ml-1">also in Java, C#, Go</span>
- **Data races:** unsynchronized access to the same data from multiple threads, at least one of them writing <span class="tag tag-yellow ml-1">also in Java, C#, Go</span>

</v-clicks>

<div v-click class="small-code mt-3">

Use-after-free in C++:

```cpp
std::vector<std::string> cart = {"Banana", "Goggles"};
std::string& first = cart[0];
cart.push_back("Overalls"); // may move the elements
std::cout << first;         // compiles, reads freed memory
```

</div>

</div>
<div class="flex flex-col gap-3 compact-stats">
  <div class="card" v-click>
    <div class="text-4xl font-700 c-red">~70%</div>
    <p>of the CVEs Microsoft assigns each year are caused by memory safety bugs <span class="muted">(Microsoft, 2019)</span></p>
  </div>
  <div class="card" v-click>
    <div class="text-4xl font-700 c-red">~70%</div>
    <p>of <strong>severe</strong> security bugs in Chromium are memory safety bugs <span class="muted">(Chromium project, 2020)</span></p>
  </div>
  <div class="card" v-click style="border-color: var(--mp-green)">
    <div class="text-4xl font-700 c-green">76% → 24%</div>
    <p>Share of memory safety bugs in Android vulnerabilities, 2019 and 2024, after moving new code to memory-safe languages <span class="muted">(Google, 2024)</span></p>
  </div>
</div>
</div>

<!--
- Use-after-free, buffer overflow and double free do not occur in Java, C# or Go. Null pointers and data races do: NullPointerException / NullReferenceException / nil pointer panic, and data races in multithreaded code. In these languages a null access throws an exception instead of corrupting memory, but the program still fails at runtime.
- Both rows show up again in the comparison on the next slide, where only Rust gets a check mark.
- Click 6, C++ example: first is a reference into the vector. push_back can move all elements to a new, larger buffer and free the old one. first then points into freed memory. It compiles without a warning (clang -Wall -Wextra), AddressSanitizer reports heap-use-after-free at runtime. The same code in Rust comes back in chapter 4 (Borrowing Rules), where the compiler rejects it.
- Also affects companies with large, experienced teams and extensive tooling
- Microsoft: ~70% of all CVEs per year, not broken down by severity. Chromium: ~70% of bugs with severity "high" or above, i.e. exactly the dangerous ones.
- Android: new code is mostly written in memory-safe languages (Java, Kotlin, Rust). Most existing C++ code was not rewritten. Most vulnerabilities come from new code, so changing the language for new code already has an effect. Source: Google Security Blog, September 2024.
- Government agencies recommend memory-safe languages: NSA (2022), CISA (2023), White House (2024)
-->

---
class: dense
clicks: 2
---

# Comparison

<div class="mt-2 tight-table" :class="{ 'hide-costs': $clicks < 1 }">

|                                  | C / C++           | Java / C# / Go     | Rust              |
| -------------------------------- | :---------------: | :----------------: | :---------------: |
| C-level performance              | <span class="c-green">✓</span> | <span class="c-yellow">~</span> | <span class="c-green">✓</span> |
| No garbage collector             | <span class="c-green">✓</span> | <span class="c-red">✘</span>    | <span class="c-green">✓</span> |
| Zero-cost abstractions           | <span class="c-green">✓</span> | <span class="c-yellow">~</span> | <span class="c-green">✓</span> |
| Memory safety                    | <span class="c-red">✘</span>   | <span class="c-green">✓</span>  | <span class="c-green">✓</span> |
| Data races detected at compile time | <span class="c-red">✘</span> | <span class="c-red">✘</span> | <span class="c-green">✓</span> |
| No `null`                        | <span class="c-red">✘</span>   | <span class="c-red">✘</span>    | <span class="c-green">✓</span> |
| <span class="muted text-sm">Costs</span> |  |  |  |
| Short compile times              | <span class="c-yellow">~</span> | <span class="c-green">✓</span> | <span class="c-red">✘</span>   |
| Gentle learning curve            | <span class="c-yellow">~</span> | <span class="c-green">✓</span> | <span class="c-red">✘</span>   |
| Readable type signatures         | <span class="c-yellow">~</span> | <span class="c-green">✓</span> | <span class="c-yellow">~</span> |

</div>

<div class="muted text-sm mt-2">Python and TypeScript belong in the middle column, with lower performance.</div>

<div v-click="2" class="mt-3">

A complex type in async code: `Pin<Box<dyn Future<Output = Result<Vec<u8>, io::Error>> + Send + 'a>>`

</div>

<style>
.tight-table tbody tr { transition: opacity 0.3s, border-color 0.3s; }
.hide-costs tbody tr:nth-child(n+7) { opacity: 0; border-color: transparent; }
</style>

<!--
- Start: only the benefits. Click 1: the last three rows (below "Costs"), the price Rust pays for the rows above. Click 2: an example of a complex type.
- Python and TypeScript belong in the middle column, with lower performance than Java, C# or Go
- Garbage collector: extra memory usage, pauses, latencies that are hard to predict
- A problem for operating systems, embedded, browsers, databases, services with latency requirements
- Rust checks memory safety at compile time. Without `unsafe`, memory bugs and data races are ruled out.
- Zero-cost abstractions: abstractions like iterators, generics or closures cost nothing at runtime compared to hand-written code. The principle comes from C++ ("zero-overhead"), Rust applies it consistently. In Java, C# or Go many abstractions have a cost (boxing, dynamic dispatch, allocations), the JIT compiler makes up for part of it. Example with iterators: chapter 6.
- Compile times: borrow checking, monomorphization of generics and LLVM optimizations take time. C is fast, C++ with templates is about as slow as Rust. Mitigations: cargo check, incremental builds, splitting into multiple crates.
- Learning curve: ownership, borrowing and lifetimes are new to most people. C is easy to learn, C++ is very large.
- The type example is only an illustration, nobody needs to understand its parts yet
- Types: usually manageable in everyday code. With async code, many trait bounds or lifetimes, signatures can get very long. Type aliases help. C++ templates have the same problem.
-->

---
class: dense
---

# Industry Adoption

<div class="grid grid-cols-4 gap-4 mt-8">
  <div class="card"><h3 class="c-orange">Amazon AWS</h3><p>Firecracker, microVMs for Lambda and Fargate</p></div>
  <div class="card"><h3 class="c-orange">Microsoft</h3><p>Parts of the Windows kernel, Azure</p></div>
  <div class="card"><h3 class="c-orange">Google</h3><p>Android, Fuchsia, Chrome (fonts, PNG decoding)</p></div>
  <div class="card"><h3 class="c-orange">Figma</h3><p>Multiplayer server for real-time collaboration</p></div>
  <div class="card"><h3 class="c-orange">Cloudflare</h3><p>Pingora, HTTP proxy for over a trillion requests per day</p></div>
  <div class="card"><h3 class="c-orange">Discord</h3><p>Backend services, partly migrated from Go</p></div>
  <div class="card"><h3 class="c-orange">Dropbox</h3><p>Sync engine of the desktop client</p></div>
  <div class="card"><h3 class="c-orange">Mozilla</h3><p>Firefox, including the CSS engine</p></div>
</div>

<div v-click class="mt-6">
Stack Overflow Developer Survey: ranked first for "most loved" or "most admired" every year from 2016 to 2025.
</div>

<!--
- Figma: the multiplayer server that synchronizes documents between users was rewritten from TypeScript to Rust in 2018 for performance (Figma blog, "Rust in production at Figma")
- Linux kernel: see the History slide
- Discord: the "Read States" service had latency spikes in Go caused by the garbage collector, after the migration it no longer did
- Stack Overflow: 2016 to 2022 "most loved", 2023 to 2025 "most admired". 2026: 67%, the highest value among widely used languages (Elm and Odin rank higher formally, but with very few responses).
- More well-known tools written in Rust: chapter 9
-->
