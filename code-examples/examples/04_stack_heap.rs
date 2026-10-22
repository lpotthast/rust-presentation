fn main() {
    let price: u32 = 450;            // stack
    let item = ("Banana", 450);      // stack (tuple)
    let sizes = ["S", "M", "L"];     // stack (array)
    let cart = vec![450, 1200];      // ptr, len, cap on the stack, elements on the heap
    let boxed = Box::new(450);       // the number on the heap
    println!("{price} {item:?} {sizes:?} {cart:?} {boxed}");
    println!("{} {} {}", size_of_val(&item), size_of_val(&cart), size_of_val(&boxed));
}
