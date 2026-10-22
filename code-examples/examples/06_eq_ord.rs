#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Money(u32); // amount in cents, u32 is Eq and Ord

#[derive(Debug, PartialEq, PartialOrd)]
struct Weight(f64); // kg, f64 is only PartialEq and PartialOrd
// #[derive(PartialEq, Eq)] on Weight: error[E0277]: the trait bound `f64: Eq` is not satisfied

fn main() {
    let nan = f64::NAN;
    println!("{}", nan == nan); // false: not reflexive, so f64 is not Eq
    println!("{}", Money(450) == Money(450)); // true
    println!("{:?}", nan.partial_cmp(&1.0)); // None: not comparable, so f64 is not Ord
    println!("{:?}", Money(450).cmp(&Money(1299))); // Less

    let mut prices = vec![Money(1299), Money(450)];
    prices.sort(); // Money is Ord
    println!("{prices:?}"); // [Money(450), Money(1299)]

    let mut weights = vec![2.5, 0.8, 1.2];
    // weights.sort(); // error[E0277]: the trait bound `{float}: Ord` is not satisfied
    weights.sort_by(f64::total_cmp);
    println!("{weights:?}"); // [0.8, 1.2, 2.5]
    println!("{}", Weight(0.8) < Weight(2.5)); // true, PartialOrd is enough for <
}
