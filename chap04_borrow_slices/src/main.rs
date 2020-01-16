fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM !!!!!!!!!!!!!!!!!!!!!!!!
    // println!("{}, {}, and {}", r1, r2, r3);
    // to solve:
    println!("{} and {}", r1, r2); //no problem
    let r3 = &mut s; // no problem
    println!("{}", r3);

    let hello = &s[0..5];  
    // or  
    let h = &s[..5];

    let world = &s[6..11];
    // or
    let w = &s[6..];

    let all = &s[0..s.len()];
    // or
    let a = &s[..];    

    let word = first_word(&s[..]);
    println!("First word: {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("First word: {}", word);

}

// the function parameter must be a refernce to the 'str' ou a slice of the 'String' types
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// change() receives a reference so 's' doesn't lost the owership
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}






// Explanations:

// Ownership and References Rules:

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
// The content of variable in the stack is copied when an assignment occur to another
// The variable in the heap is transferred when an assignment occur but it can be borrowed through
//  a reference prefix '&'
// You can have only one mutable reference to a particular piece of data in a particular scope
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
// References does not have owership
// Another data type that does not have ownership is the slice
