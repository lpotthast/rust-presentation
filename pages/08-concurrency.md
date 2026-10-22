---
layout: section
---

<div class="chapter">Chapter 8</div>

# Threads & Async

---
class: dense
---

# Threads

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|9-10|9-10|11-12,16-18|all}
use std::thread;
fn main() {
    let orders = vec![
        vec![450, 1200], // prices in cents
        vec![299],
        vec![1299, 450, 99],
    ];
    let mut handles = vec![];
    for order in orders {
        handles.push(thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            order_total
        }));
    }

    let mut total = 0;
    for h in handles {
        total += h.join().unwrap();
    }
    println!("Total: {total} cents"); // 3797
}
```

</div>
<div class="text-base">

<div v-click="1">

`thread::spawn` starts a thread with a closure. `move` transfers ownership of `order` into the thread.

</div>

<div v-click="2" class="mt-4">

`thread::spawn` requires that the closure owns its data (`'static`), because the thread can outlive the current function.

</div>

<div v-click="3" class="mt-4">

`join()` waits for the thread and returns the value of the closure.

</div>

</div>
</div>

---
class: dense
---

# Data Races

Two threads add products to the same list at the same time.

<div class="grid grid-cols-[2fr_3fr] gap-8">
<div>

<div class="relative">

```rust {all|5|6|all}{at:2}
use std::thread;
fn main() {
    let mut sold = Vec::new();
    thread::scope(|s| {
        s.spawn(|| sold.push("Banana"));
        s.spawn(|| sold.push("Goggles"));
    });
}
```

<div class="code-pointer" style="top: 90px">▶</div>
</div>

<div class="text-base mt-3">

`thread::scope`: threads that end before the scope ends. They may borrow local variables.

</div>

<div v-click="5" class="text-base mt-3">

Both closures need `&mut sold` at the same time. The rule from Borrowing Rules (one `&mut` or many `&`) rules out the data race.

</div>

</div>
<div v-click="1" class="small-code">

```text {1-2|5-10|11-14|all}{at:2}
error[E0499]: cannot borrow `sold` as mutable more than once at a time
   --> src/main.rs:6:17
    |
...
  5 |         s.spawn(|| sold.push("Banana"));
    |         -------------------------------
    |         |       |  |
    |         |       |  first borrow occurs due to use of `sold` in closure
    |         |       first mutable borrow occurs here
    |         argument requires that `sold` is borrowed for `'1`
  6 |         s.spawn(|| sold.push("Goggles"));
    |                 ^^ ---- second borrow occurs due to use of `sold` in closure
    |                 |
    |                 second mutable borrow occurs here
```

</div>
</div>

---
class: dense
---

# Shared State with `Arc` and `Mutex`

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|9|12-13|15|all}
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let orders = vec![
        vec![450, 1200], // prices in cents
        vec![299],
        vec![1299, 450, 99],
    ];
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for order in orders {
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            *total.lock().unwrap() += order_total;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Total: {} cents", *total.lock().unwrap()); // 3797
}
```

</div>
<div class="text-base arc-notes">

<div v-click="1" class="arc-diagram">
  <div></div>
  <div class="c-blue font-600 text-sm">Stack</div>
  <div></div>
  <div class="c-yellow font-600 text-sm">Heap</div>
  <div class="muted text-sm">main</div>
  <code>total: Arc</code>
  <div class="arc-arrow">↘</div>
  <code class="arc-heap">count: {{ $clicks >= 2 && $clicks < 4 ? 4 : 1 }}<br>Mutex({{ $clicks >= 3 ? 3797 : 0 }})</code>
  <template v-for="n in 3" :key="n">
    <div v-click="2" class="muted text-sm">thread {{ n }}</div>
    <code v-click="2" :class="{ 'line-through': $clicks >= 4 }" :style="$clicks >= 4 ? 'color: var(--mp-red)' : ''">total: Arc</code>
    <div v-click="[2, 4]" class="arc-arrow">↗</div>
  </template>
</div>

<div v-click="2" class="mt-2">

`Arc::clone` copies only the pointer and increases the count.

</div>

<div v-click="3" class="mt-2">

`Mutex`: the value is only reachable through `lock()`.

</div>

<div v-click="4" class="mt-2">

Each thread drops its `Arc` when it ends. At count 0 the value is freed.

</div>

<div v-click="5" class="mt-2">

With `Rc`, the single-threaded variant of `Arc`, this does not compile.

</div>

</div>
</div>

<style>
.arc-diagram {
  display: grid;
  grid-template-columns: auto auto auto auto;
  align-items: center;
  justify-content: start;
  gap: 0.3rem 0.8rem;
  white-space: nowrap;
}
.arc-arrow {
  color: var(--mp-fg);
  font-size: 1.2rem;
}
.arc-notes p { margin: 0.5rem 0 0; }
.arc-heap {
  grid-row: 2 / span 4;
  grid-column: 4;
  align-self: center;
}
</style>

---
class: dense
---

# `Send` & `Sync`

<div class="grid grid-cols-2 gap-8">
<div>

<div class="relative">

```rust {1,2,4}
// Shared State slide, with Rc instead of Arc
let total = Rc::new(Mutex::new(0));
for order in orders {
    let total = Rc::clone(&total);
    handles.push(thread::spawn(move || {
        let order_total: u32 = order.iter().sum();
        *total.lock().unwrap() += order_total;
    }));
}
```

<div class="code-pointer" v-click="1" style="top: 72px">▶</div>
</div>

<div v-click="1" class="small-code">

```text {1-2|9-11|12|all}{at:2}
error[E0277]: `Rc<std::sync::Mutex<u32>>` cannot be
              sent between threads safely
  --> src/main.rs:16:36
   |
16 |         handles.push(thread::spawn(move || {
   |                      ------------- required by a bound
   |                                    introduced by this call
...
   = help: within `{closure@src/main.rs:16:36: 16:43}`,
           the trait `Send` is not implemented
           for `Rc<std::sync::Mutex<u32>>`
note: required by a bound in `spawn`
...
```

</div>

</div>
<div>

<div v-click="4">

| Trait  | Meaning |
| ------ | --------- |
| `Send` | may be moved to another thread |
| `Sync` | references (`&T`) may be shared between threads |

</div>

<div v-click="5" class="mt-4 tight-table">

| Type                        | `Send` | `Sync` |
| --------------------------- | :----: | :----: |
| `i32`, `String`, `Vec<i32>` | <span class="c-green">✓</span> | <span class="c-green">✓</span> |
| `Arc<T>`, `Mutex<T>`        | <span class="c-green">✓</span> | <span class="c-green">✓</span> |
| `Cell<T>`, `RefCell<T>`     | <span class="c-green">✓</span> | <span class="c-red">✘</span> |
| `MutexGuard<T>`             | <span class="c-red">✘</span> | <span class="c-green">✓</span> |
| `Rc<T>`                     | <span class="c-red">✘</span> | <span class="c-red">✘</span> |

</div>

<p v-click="6" class="text-base" style="margin: 0.5rem 0 0">
<code>Rc</code> updates its count without atomic operations, so it is not <code>Send</code>. The compiler derives <code>Send</code> and <code>Sync</code> from a type's fields.
</p>

</div>
</div>

---
class: dense
---

# Channels

The same task again. Instead of sharing a total, each thread sends its result.

<div class="grid grid-cols-[3fr_2fr] gap-8">
<div>

```rust {all|9|11-12|14|17|19|all}
use std::sync::mpsc;
use std::thread;
fn main() {
    let orders = vec![
        vec![450, 1200], // prices in cents
        vec![299],
        vec![1299, 450, 99],
    ];
    let (tx, rx) = mpsc::channel(); // tx: sender, rx: receiver
    for order in orders {
        let tx = tx.clone();
        thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            tx.send(order_total).unwrap();
        });
    }
    drop(tx); // the channel closes when all senders are gone

    let total: u32 = rx.iter().sum();
    println!("Total: {total} cents"); // 3797
}
```

</div>
<div class="text-base channel-notes">

<div v-click="1">

`mpsc`: multiple producers, single consumer. `channel()` returns a sender and a receiver.

</div>
<div v-click="2">

Each thread gets its own sender.

</div>
<div v-click="3">

`send` moves the value into the channel, the receiver becomes its owner.

</div>
<div v-click="4">

The channel closes when all senders are dropped, including the one in `main`.

</div>
<div v-click="5">

`rx.iter()` returns the messages until the channel is closed.

</div>
<div v-click="6">

In async code, `tokio::sync::mpsc` works the same way, with `.await` on `send` and `recv`.

</div>

</div>
</div>

<style>
.channel-notes p { margin: 0.5rem 0 0; }
</style>

---

# Async

<div class="grid grid-cols-2 gap-8 mt-6">
<div>

<v-clicks>

- Many programs spend most of their time waiting for I/O: network, database, file system
- One OS thread per connection costs memory and context switches
- With async, a thread works on other tasks while one task is waiting
- Typical applications: web servers, proxies, microservices

</v-clicks>

</div>
<div class="flex flex-col gap-4">
  <div class="card" v-click>
    <h3 class="c-orange">No built-in runtime</h3>
    <p>The standard library only defines the traits (e.g. <code>Future</code>). The runtime is a library, usually <strong>Tokio</strong>, for embedded e.g. <strong>Embassy</strong>.</p>
  </div>
  <table v-click class="text-base">
    <tbody>
      <tr><td class="whitespace-nowrap">10,000 threads</td><td class="whitespace-nowrap">10,000 stacks of 2 MiB each</td></tr>
      <tr><td class="whitespace-nowrap">10,000 Tokio tasks</td><td class="whitespace-nowrap">only the state of each future</td></tr>
    </tbody>
  </table>
</div>
</div>

---
class: dense
---

# Futures

A `Future` is a value for a result that is not available yet, comparable to a `Promise` in JavaScript or a `Task` in C#.

<div class="grid grid-cols-2 gap-8 mt-2">
<div>

```rust {1|2-6|all}
fn fetch_user(id: u32) -> impl Future<Output = String> {
    async move {
        // simulates a database query
        sleep(Duration::from_millis(100)).await;
        format!("user {id}")
    }
}
```

<div v-click="2" class="mt-4">

```rust
async fn fetch_user(id: u32) -> String {
    // simulates a database query
    sleep(Duration::from_millis(100)).await;
    format!("user {id}")
}
```

</div>

</div>
<div class="text-base">

`impl Future<Output = String>`: some type that implements the `Future` trait and produces a `String` when it completes.

<div v-click="1">

An `async` block creates such a future. Inside it, `.await` waits for another future without blocking the thread.

</div>

<div v-click="2" class="mt-4">

`async fn` is syntactic sugar for exactly this: a function that returns a future.

</div>

<div v-click="3" class="mt-4">

Futures are lazy. Unlike a JavaScript `Promise` or a C# `Task`, a future does nothing until it is awaited or handed to the runtime (`tokio::spawn`).

```rust
let future = fetch_user(1); // nothing runs yet
let user = future.await;    // now it runs
```

</div>

</div>
</div>

---
class: dense
---

# async / await with Tokio

<div class="grid grid-cols-2 gap-8">
<div>

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_user(id: u32) -> String {
    // simulates a database query
    sleep(Duration::from_millis(100)).await;
    format!("user {id}")
}

async fn fetch_orders(user_id: u32) -> Vec<u32> {
    // simulates an HTTP request
    sleep(Duration::from_millis(100)).await;
    vec![user_id * 10, user_id * 10 + 1]
}
```

</div>
<div>

```rust {all|1-2|3-4|6-7|9-11|13-14|16-19|all}
#[tokio::main]
async fn main() {
    // sequential
    let user = fetch_user(1).await;

    // separate task, starts running in the background
    let handle = tokio::spawn(fetch_orders(1));

    // concurrent: both wait at the same time
    let (other_user, other_orders) =
        tokio::join!(fetch_user(2), fetch_orders(2));

    // wait for the result of the task
    let orders = handle.await.unwrap();

    println!("{user}: {orders:?}");
    println!("{other_user}: {other_orders:?}");
    // user 1: [10, 11]
    // user 2: [20, 21]
}
```

</div>
</div>

---

# Execution over Time

<div class="timechart mt-6">
  <div class="tc-axis"><div class="tc-track"><span style="left:0%">0 ms</span><span style="left:25%">100 ms</span><span style="left:50%">200 ms</span><span style="left:75%">300 ms</span><span style="left:100%">400 ms</span></div></div>

  <div class="tc-group"><code>main</code> from the previous slide</div>
  <div class="tc-row" v-click="1"><span class="tc-label">fetch_user(1)</span><div class="tc-track"><div class="tc-bar tagged c-bg-blue" style="left:0%;width:25%">.await</div></div></div>
  <div class="tc-row" v-click="2"><span class="tc-label">fetch_orders(1)</span><div class="tc-track"><div class="tc-bar tagged c-bg-orange" style="left:25%;width:25%">tokio::spawn</div></div></div>
  <div class="tc-row" v-click="3"><span class="tc-label">fetch_user(2)</span><div class="tc-track"><div class="tc-bar tagged c-bg-green" style="left:25%;width:25%">join!</div></div></div>
  <div class="tc-row" v-click="3"><span class="tc-label">fetch_orders(2)</span><div class="tc-track"><div class="tc-bar tagged c-bg-green" style="left:25%;width:25%">join!</div></div></div>
  <div class="tc-total" v-click="3"><div class="tc-track"><div class="tc-mark" style="left:50%">~200 ms</div></div></div>

  <div class="tc-group mt-6" v-click="4">Everything in sequence with <code>.await</code></div>
  <div class="tc-row" v-click="4"><span class="tc-label">fetch_user(1)</span><div class="tc-track"><div class="tc-bar tagged c-bg-dim" style="left:0%;width:25%">.await</div></div></div>
  <div class="tc-row" v-click="4"><span class="tc-label">fetch_orders(1)</span><div class="tc-track"><div class="tc-bar tagged c-bg-dim" style="left:25%;width:25%">.await</div></div></div>
  <div class="tc-row" v-click="4"><span class="tc-label">fetch_user(2)</span><div class="tc-track"><div class="tc-bar tagged c-bg-dim" style="left:50%;width:25%">.await</div></div></div>
  <div class="tc-row" v-click="4"><span class="tc-label">fetch_orders(2)</span><div class="tc-track"><div class="tc-bar tagged c-bg-dim" style="left:75%;width:25%">.await</div></div></div>
  <div class="tc-total" v-click="4"><div class="tc-track"><div class="tc-mark" style="left:100%">~400 ms</div></div></div>
</div>

<div v-click="5" class="mt-6">

While a task is waiting, the thread keeps working on other tasks. One thread per request is not needed.

</div>

<style>
.timechart .tc-bar.tagged {
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--slidev-code-font-family);
  font-size: 0.65rem;
  color: var(--mp-bg);
}
</style>

---
class: dense
---

# `Send` in Async Code

<div class="grid grid-cols-2 gap-8">
<div>

<div class="relative">

```rust
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

async fn process_order() {
    let order_id = Rc::new(1001); // Rc is not Send
    sleep(Duration::from_millis(10)).await;
    println!("order {order_id}");
}

#[tokio::main]
async fn main() {
    tokio::spawn(process_order()).await.unwrap();
}
```

<div class="code-pointer" v-click="1" :style="{ top: ($clicks === 2 ? 104 : 198) + 'px' }">▶</div>
</div>

<ul class="mt-2">
<li v-click="4"><code>tokio::spawn</code> requires <code>Future + Send + 'static</code></li>
<li v-click="5">Tasks can switch threads at every <code>.await</code></li>
<li v-click="6">Values that live across an <code>.await</code> are stored in the future, so they must be <code>Send</code></li>
</ul>

</div>
<div>

<div v-click="1" class="small-code">

```text {1-6|8-17|all}{at:2}
error: future cannot be sent between threads safely
  --> src/main.rs:13:18
   |
13 |     tokio::spawn(process_order()).await.unwrap();
   |                  ^^^^^^^^^^^^^^^ future returned by
   |                  `process_order` is not `Send`
...
note: future is not `Send` as this value is used
      across an await
  --> src/main.rs:7:38
   |
 6 |     let order_id = Rc::new(1001); // Rc is not Send
   |         -------- has type `Rc<i32>` which is not `Send`
 7 |     sleep(Duration::from_millis(10)).await;
   |                                      ^^^^^ await occurs
   |                                      here, with `order_id`
   |                                      maybe used later
```

</div>

<div v-click="7">

**Solutions**

- `Arc` instead of `Rc`
- Limit the value to a block that ends before the `.await`
- `tokio::sync::Mutex` for locks held across an `.await`

</div>

</div>
</div>
