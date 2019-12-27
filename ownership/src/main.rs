fn main() {
    main_before_splice();

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");
    // these are the same
    let slice = &s[0..2];
    let slice = &s[..2];


    let s = String::from("hello");
    let len = s.len();
    // these are the same
    let slice = &s[3..len];
    let slice = &s[3..];


    let s = String::from("hello");
    let len = s.len();
    // these are the same
    let slice = &s[0..len];
    let slice = &s[..];


    let mut s = String::from("hello world");
    let word = first_word(&s);
//    s.clear(); we couldn't do this as word is still just a reference to s and we are trying
    // to use it below
    println!("word: {}", word);


    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}

fn main_before_splice() {
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
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;


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
