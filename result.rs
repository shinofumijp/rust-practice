fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("Invalid value"))
    }
}

fn main() {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("Found: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
