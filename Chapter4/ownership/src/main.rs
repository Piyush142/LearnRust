fn main() {
    let mut s = String::from("world");
    s = String::from("something else");
    println!("Hello, {}!", s);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it's okay to use x afterward.

    let mut s3 = String::from("hello");

    change(&mut s3);
    let word = first_word(&s3);
    println!("The first word is: {}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
