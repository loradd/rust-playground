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

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rectangle);

    let inner_rectangle = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    println!("{}", rectangle.can_hold(&inner_rectangle));
}
