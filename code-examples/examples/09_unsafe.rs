unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

// safe API around the C function
fn c_abs(x: i32) -> Option<i32> {
    if x == i32::MIN {
        return None; // undefined behavior in C
    }
    Some(unsafe { abs(x) })
}

fn main() {
    println!("{:?}", c_abs(-3)); // Some(3)
    println!("{:?}", c_abs(i32::MIN)); // None
}
