fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.", area(&rectangle)
    );
}

// use & as we don't want to change ownership
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

struct Rectangle {
    width: u32,
    height: u32,
}
