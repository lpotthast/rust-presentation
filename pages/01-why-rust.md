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

---
class: dense
---

# Memory Bugs in C and C++

<div class="grid grid-cols-2 gap-10 mt-6">
<div>

Typical bug classes in C and C++:

<v-clicks>

- **Use-after-free:** accessing already freed memory
- **Buffer overflow:** writing past the end of an array
- **Double free:** memory is freed twice
- **Null pointer:** access through a pointer without a valid target <span class="tag tag-yellow ml-1">also in Java, C#, Go</span>
- **Data races:** unsynchronized access to the same data from multiple threads, at least one of them writing <span class="tag tag-yellow ml-1">also in Java, C#, Go</span>

</v-clicks>

<div v-click class="mt-3">

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
