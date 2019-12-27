fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces count is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Usage: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Usage: {} {}", x, y);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("Usage: {} {} {} {} {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Usage: {} {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Usage: {} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x y z is: {} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Usage: {} {} {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    let index = 10;
//    let element = a[index];// expected error
//    println!("The value of element is: {}", element);
}
