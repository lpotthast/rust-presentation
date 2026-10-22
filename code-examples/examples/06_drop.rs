struct Reservation {
    item: String,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        // e.g. put the item back in stock
        println!("released {}", self.item);
    }
}

fn main() {
    {
        let r = Reservation { item: String::from("Banana") };
        println!("reserved {}", r.item);
    } // the compiler calls drop here
    println!("done");
}
