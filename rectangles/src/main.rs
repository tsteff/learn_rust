fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1); // the hash makes prettyprint
}

// use & as we don't want to change ownership
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
