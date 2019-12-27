fn main() {
    let mut ss = String::from("hello");
    ss.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", ss); // This will print `hello, world!`

    // makes copy of x before putting in y
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // this isn't possible as s1 is invalidated when s2 is created using it
//    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function...
    takes_ownership(s);
    // ... and so is no longer valid here
//    println!("s = {}", s);


    // x comes into scope
    let x = 5;

    // x would move into the function,
    makes_copy(x);
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    println!("x = {}", x);


    // wrong way
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_wrong_way(s1);
    println!("The length of '{}' is {}.", s2, len);

    // correct way
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // example of mutable when passing to another method
    let mut s = String::from("hello");
    change(&mut s);


    // can't have more than 1 mutable reference, this works as {} is another scope
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    // no problem because r1 and r2 aren't used again. If they were then fail
    let r3 = &mut s;
    println!("{}", r3);
}

    // some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length_wrong_way(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
