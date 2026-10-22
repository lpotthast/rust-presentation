---
layout: section
---

<div class="chapter">Chapter 8</div>

# Threads & Async

<!--
- Builds on ownership (chapter 4) and closures (chapter 6)
-->

---
class: dense
---

# Smart Pointers & Mutex

For cases where plain ownership is not enough, e.g. data shared by several threads:

| Type       | Use                                                          | Comparable to |
| ---------- | ------------------------------------------------------------ | ------------- |
| `Box<T>`   | Value on the heap with one owner, e.g. `Box<dyn Error>`      | C++ unique_ptr |
| `Rc<T>`    | Shared ownership via reference counting, one thread only     | C++ shared_ptr, non-atomic |
| `Arc<T>`   | Like `Rc`, with atomic reference counting, across threads    | C++ shared_ptr |
| `Mutex<T>` | Mutable access from several threads, owns the data           | Java synchronized, C# lock |

<div class="grid grid-cols-[3fr_2fr] gap-8 mt-4">
<div>

<div v-click="1">

In Java and C#, every object reference is shared and the garbage collector frees the object. Rust makes shared ownership explicit.

</div>

<div v-click="2" class="mt-2">

The data inside a `Mutex<T>` can only be reached through the lock. Common combination for shared mutable state between threads: `Arc<Mutex<T>>`

</div>

</div>
<div v-click="2" class="arc-diagram">
  <div class="c-blue font-600 text-sm">Stack</div>
  <div></div>
  <div class="c-yellow font-600 text-sm">Heap</div>
  <code>total: Arc</code>
  <div class="arc-arrow">↘</div>
  <code class="arc-heap">count: 2 | Mutex(0)</code>
  <code>total: Arc</code>
  <div class="arc-arrow">↗</div>
</div>
</div>

<style>
.arc-diagram {
  display: grid;
  grid-template-columns: auto auto auto;
  align-items: center;
  justify-content: start;
  gap: 0.4rem 0.8rem;
}
.arc-arrow {
  color: var(--mp-fg);
  font-size: 1.2rem;
}
.arc-heap {
  grid-row: 2 / span 2;
  grid-column: 3;
  align-self: center;
}
</style>

<!--
- Vec and String manage their heap memory on the same principle
- Rc and Arc free the data when the last pointer leaves its scope
- Rc/Arc and ownership rule 1: each Rc pointer has exactly one owner, several Rc pointers share the value behind them
- Box<dyn Error> from chapter 7 (? operator): the error has a size unknown at compile time, Box puts it on the heap
- Mutex<T>: the protected value lives inside the mutex. Forgetting to lock is impossible, unlike a Java field guarded by synchronized by convention.
- More types if needed: RefCell<T> checks the borrowing rules at runtime instead of at compile time
- Click 2, diagram: two Arc pointers (e.g. in two threads) share one heap block with the reference count and the Mutex. The last Arc that is dropped frees the block.
- Arc<Mutex<T>> is used by the thread example on the next slide. Why Rc does not work across threads: slide Send & Sync.
-->

---
class: dense
---

# Threads

<div class="grid grid-cols-[5fr_2fr] gap-6">
<div>

```rust {all|5-6|9-10|11-14|17-19|20|all}
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let orders = vec![vec![450, 1200], vec![299], vec![1299, 450, 99]];
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
<div class="muted text-base">

<div v-click="2">

Each thread gets its own `Arc` pointer. `move` transfers ownership of `order` and this pointer into the thread. `thread::spawn` requires that the closure owns its data (`'static`), because the thread can outlive the current function.

</div>

<div v-click="3" class="mt-4">

The value inside the mutex is only accessible through `lock()`. The lock is released when the guard returned by `lock()` is dropped, here at the end of the statement.

</div>

<div v-click="6" class="mt-4">

With `Rc` instead of `Arc`, the code does not compile.

</div>

</div>
</div>

<!--
- "Fearless Concurrency": term from the official Rust blog (2015)
- The same ownership rules prevent memory bugs and data races
- Example: add up the totals of several orders in parallel, amounts in cents
- move || { ... } is a closure without parameters. move transfers ownership of the variables it uses (order, total) into the thread.
- Arc::clone only copies the pointer and increments the reference count, not the data
- lock() returns a guard, * accesses the value behind it
- join() waits for the thread to finish. It returns an Err if the thread panicked, unwrap() passes that panic on.
- Click 2: Arc::clone and move, together with the explanation on the right. Click 3: lock() together with its explanation. Click 6: teaser for the next slide.
- lock() returns a Result: it is an Err if another thread panicked while holding the lock (the mutex is "poisoned"). unwrap() passes that panic on.
- thread::scope (since Rust 1.63) lets threads borrow local data without Arc, because all threads end before the scope ends
- Why Rc does not compile: next slide, with the real error message
-->

---
class: dense
---

# `Send` & `Sync`

<div class="grid grid-cols-2 gap-8">
<div>

<div class="relative">

```rust {1,2,4}
// Threads slide, with Rc instead of Arc
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
  --> src/main.rs:12:36
   |
12 |         handles.push(thread::spawn(move || {
   |                      ------------- required by a bound
   |                                    introduced by this call
...
   = help: within `{closure@src/main.rs:12:36: 12:43}`,
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

<!--
- The code from the Threads slide, only Arc replaced by Rc (highlighted lines). Ask: does this compile?
- Click 1 to 4: the error message, step by step. The arrow marks the thread::spawn line, where rustc reports the error (12:36). thread::spawn requires the closure to be Send. The closure contains an Rc, and Rc is not Send.
- Error message from rustc, shortened ("...") and wrapped for the slide. Full version: code-examples/compile-errors/08_rc_thread.rs
- Sync: Mutex<T> is Sync, although threads change the value through &Mutex. The mutex makes sure only one thread at a time gets access.
- Why: the reference count of Rc is not atomic. Two threads changing it at the same time would be a data race. Arc counts atomically.
- Fix: Arc instead of Rc, as on the Threads slide
- Send and Sync are marker traits without methods. thread::spawn and tokio::spawn require Send as a trait bound.
- Arc<T> is only Send and Sync if T: Send + Sync. Mutex<T> is Send and Sync if T: Send. That way Mutex makes a type that is only Send also Sync.
- The table assumes a simple T such as i32. Generic types inherit the properties of T.
- Cell and RefCell: Send, but not Sync. They allow changing a value through a shared reference (interior mutability) and check borrowing at runtime without synchronization. Moving them to another thread is fine, using them from two threads at the same time is not.
- MutexGuard: Sync, but not Send. lock() returns it (Threads slide). The lock must be released on the same thread that acquired it, because some operating systems require this. Sharing &MutexGuard is harmless, it only allows reading.
- 'static in the spawn signature: the closure must not contain borrowed references with a shorter lifetime (slide Lifetime Annotations, chapter 4)
-->

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

<!--
- async/await is familiar from JavaScript, C# and Python
- Thread cost: every thread spawned by Rust reserves its own stack, 2 MiB by default (virtual memory, physical pages only when used), plus a context switch for every change.
- Last click: the comparison. A task stores only the variables that live across an .await, often a few hundred bytes.
- Java 21 virtual threads solve the same problem inside the JVM. In Rust, async tasks take this role.
- In JavaScript the event loop is part of the runtime (browser, Node.js)
- Without a built-in runtime, async also works on microcontrollers without an operating system
-->

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

<!--
- Start: line 1 highlighted, with the explanation of impl Future. Click 1: the async block (lines 2-6) and its explanation.
- The long form: a normal function that returns a future. The return type says what the caller gets: not a String, but something that will produce a String later.
- impl Trait in the return type: "some type that implements this trait", the concrete type is generated by the compiler. Same idea as &impl Notifier in parameters (chapter 6).
- async move { ... }: an async block. move takes ownership of id, like a move closure.
- Click 2: the same function written as async fn, below the long form. Both versions are equivalent, async fn is the usual way to write it. Clippy even suggests the rewrite for the long form (lint manual_async_fn).
- Future is in the prelude since edition 2024, no import needed. sleep and Duration come from tokio and std (full code on the next slide).
- Click 3: calling fetch_user only creates the future. The work happens at .await. In JavaScript and C#, calling an async function starts it immediately. Python coroutines are lazy like Rust futures.
- Lazy like iterators (chapter 6): nothing happens until something drives them
- Under the hood the compiler turns the async block into a state machine (see slide Execution over Time).
-->

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

<!--
- Example: load user 1 and their orders, and user 2 with their orders. sleep stands for a database query or an HTTP request.
- The orders of user 1 load in a background task while user 2 and their orders load with join!
- #[tokio::main] starts the runtime and runs main inside it
- Takes about 200 ms instead of 400 ms: the spawned task and the two calls in join! wait at the same time (next slide)
- tokio::spawn starts the future immediately as its own task. The handle can be awaited later to get the result. handle.await returns an Err if the task panicked, unwrap() passes that on.
- Tokio tasks are very lightweight, hundreds of thousands of tasks are possible
- The default runtime distributes tasks across several worker threads
-->

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

<!--
- Diagram for the code on the previous slide, each call waits 100 ms (simulated I/O)
- Each bar shows the mechanism that runs the call: .await (blue), tokio::spawn (orange), join! (green)
- Click 1: fetch_user(1) runs alone, everything else waits for the .await
- Click 2: tokio::spawn starts fetch_orders(1) as a separate task. It runs while main continues.
- Click 3: join! starts both futures and waits for both at the same time. All three take as long as the slowest one.
- Click 4: for comparison, the same four calls in sequence
- Under the hood: the compiler turns every async fn into a state machine, similar to an enum with one variant per section between two .await points. The runtime polls the futures with poll(). The language itself needs neither a garbage collector nor heap allocations for this.
- That is why it also works on microcontrollers with a few KB of RAM (Embassy)
-->

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

<!--
- Optional
- Show the code first and ask: does this compile? From click 1 the arrow marks tokio::spawn, where rustc reports the error.
- Click 1: the error at the spawn call. Click 2: the note, the arrow moves to the .await in line 7, where data is still alive. Click 3: whole message.
- Clicks 4 to 6: why. The future stores every value that is still needed after an .await (state machine, slide Execution over Time). If such a value is not Send, the future is not Send either. Click 7: solutions.
- 'static: the future must not borrow local variables of the caller, because the task can outlive them. It must own its data (slide Lifetime Annotations, chapter 4).
- Second solution: { let order_id = Rc::new(1001); println!("order {order_id}"); } before the .await. An explicit drop(order_id) is not enough, the compiler still treats data as live across the .await (checked with Rust 1.99).
- Tokio distributes tasks across several worker threads. A task can pause at any .await and continue on a different thread.
- The guard of std::sync::Mutex is not Send, so it must not be held across an .await
- The most common source of errors when starting with async Rust
- In general, async is considered one of the more complex parts of Rust: long error messages for Send and lifetime problems, and async and sync code cannot be combined freely. Simple programs usually do not need it.
- Improvements in recent years: async fn in traits since Rust 1.75, async closures since 1.85
- Error message from rustc, shortened ("...") and wrapped for the slide. Full version: code-examples/compile-errors/08_rc_not_send.rs (needs tokio, e.g. copy it to examples/)
-->

---
class: dense
---

# Channels

Tasks can also send each other messages instead of sharing state with `Arc<Mutex<T>>`.

<div class="grid grid-cols-[3fr_2fr] gap-8 mt-2">
<div>

```rust {all|5|7-15|12|16|18-20|all}
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // tx: sender, rx: receiver

    for customer in ["Kevin", "Stuart"] {
        let tx = tx.clone();
        tokio::spawn(async move {
            for id in 1..=2 {
                let order = format!("order {id} from {customer}");
                tx.send(order).await.unwrap();
            }
        });
    }
    drop(tx); // the channel closes when all senders are gone

    while let Some(order) = rx.recv().await {
        println!("processing {order}");
    }
}
```

</div>
<div>

<div v-click="6" class="small-code">

```text
processing order 1 from Kevin
processing order 1 from Stuart
processing order 2 from Stuart
processing order 2 from Kevin
```

</div>

<div v-click="7" class="text-base mt-4">

`mpsc`: multiple producers, single consumer. `send` moves the message into the channel, the receiving task becomes its owner.

The order differs between runs. For threads there is `std::sync::mpsc`, with blocking `send` and `recv` instead of `.await`.

</div>

</div>
</div>

<!--
- Optional
- Click 1: a channel with room for 32 messages. tx is the sending end, rx the receiving end (comment in the code).
- Click 2: two tasks, one per customer, each with its own copy of tx
- Click 3: send moves the String into the channel. If the channel is full, send waits (backpressure).
- Click 4: main drops its own tx. Otherwise the channel would stay open and the loop below would never end.
- Click 5: recv returns None once all senders are gone, which ends the while let loop
- Click 6: real output of cargo run --example 08_channels. Kevin and Stuart run concurrently, the order changes between runs.
- Instead of several tasks writing to shared data under a lock, one task owns the data and the others send it messages. Go describes this as "share memory by communicating".
- std::sync::mpsc::channel() is unbounded, std::sync::mpsc::sync_channel(n) has a fixed capacity. Both block the thread instead of awaiting.
- Other channel types in tokio::sync: oneshot (exactly one value), broadcast (every receiver gets every message), watch (latest value only)
-->
