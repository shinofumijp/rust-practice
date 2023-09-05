struct BagOfHolding<T> {
    item: Option<T>,
}

fn main() {
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("Nothing in the bag")
    } else {
        println!("Something in the bag")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("Something in the bag")
    } else {
        println!("Nothing in the bag")
    }

    match i32_bag.item {
        Some(v) => println!("{} found in the bag", v),
        None => println!("Nothing found"),
    }
}
