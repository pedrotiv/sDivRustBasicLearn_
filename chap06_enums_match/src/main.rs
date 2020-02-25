// Enums allow you to define a type by enumerating its possible variants.
// Rust’s enums are most similar to algebraic data types in functional languages

//A Message enum whose variants each store different amounts and types of values
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//this is similar to defining different kinds of struct definitions for each variants
// But if we used the different structs, which each have their own type, we couldn’t as
// easily define a function to take any of these kinds of messages
impl Message {
    fn call(&self) {
        println!("Hi, I'm called");        
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option<T> enum is so useful that it’s even included in the prelude; 
    // you don’t need to bring it into scope explicitly. In addition, so are its variants:
    //  you can use Some and None directly without the Option:: prefix. 
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
    println!("{:?}, {:?}, {:?}", some_number.unwrap(), some_string.unwrap(), absent_number);


}
