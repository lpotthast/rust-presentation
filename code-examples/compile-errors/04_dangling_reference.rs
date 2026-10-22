fn main() {
    let r;
    {
        let s = String::from("hello");
        r = &s;
    } // s is dropped here
    println!("{r}");
}
