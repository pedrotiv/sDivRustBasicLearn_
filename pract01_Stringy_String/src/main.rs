#![allow(non_snake_case)]
// A view of borrow and own.alloc
// If I own something, you can borrow it, but you can’t change it.


fn main() {
    let literal: &str = "some literal str";
    let actual_String: String = String::from("some String");
    borrowed_str(literal);    
    // transfer ownership to function
    purchased_String(actual_String);
    println!("{}",literal);
    // println!("{}",actual_String); If we uncomment this code, the code doesn't compile 
    //               ^^^^^^^^^^^^^ value borrowed here after move
    // 
}

fn borrowed_str(stringy: &str){
    // drop do nothing because it doesn't drop any borrewed value.
    drop(stringy);
}

fn purchased_String(stringy: String){
    // stringy will be droped. 
    drop(stringy);
}

