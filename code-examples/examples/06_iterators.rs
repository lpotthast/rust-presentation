use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// counts heap allocations, to check the claim on the slide
struct CountingAlloc;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

fn main() {
    let input = "summer, sale,, kids , extra-waterproof"; // e.g. from a form field
    let max_len = 10; // captured by the closure

    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let tags: Vec<&str> = input
        .split(',')
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty() && tag.len() <= max_len)
        .collect(); // ["summer", "sale", "kids"]
    let with_collect = ALLOCATIONS.load(Ordering::Relaxed) - before;

    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let count = input
        .split(',')
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty() && tag.len() <= max_len)
        .count(); // 3
    let with_count = ALLOCATIONS.load(Ordering::Relaxed) - before;

    println!("{tags:?}"); // ["summer", "sale", "kids"]
    println!("{count}"); // 3
    println!("allocations: collect {with_collect}, count {with_count}"); // collect 1, count 0
}
