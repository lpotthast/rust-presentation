fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 is now the owner (move)
    // println!("{s1}"); // error: s1 is no longer valid
    println!("{s2}");

    let s3 = s2.clone(); // explicit deep copy
    take(s3); // the function becomes the owner
    // s3 is no longer valid here

    let a = 42;
    let b = a; // i32 is Copy and is simply copied
    println!("{a} {b}");
} // s2 is dropped here

fn take(s: String) {
    println!("{s}");
} // s is dropped here

// ownership rules: implicit drop at the end of each scope
#[allow(dead_code)]
fn scopes() {
    let order = String::from("A-1001");
    {
        let label = format!("Label {order}");
        println!("{label}");
        drop(label); // inserted by the compiler
    }
    println!("{order}");
    drop(order); // inserted by the compiler
}
