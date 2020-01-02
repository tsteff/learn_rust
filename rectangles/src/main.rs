fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1); // the hash makes prettyprint
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );    
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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}