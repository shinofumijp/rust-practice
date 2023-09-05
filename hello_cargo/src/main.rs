#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 42,
        height: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("rect is {:#?}", rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
