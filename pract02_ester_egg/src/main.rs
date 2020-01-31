// As in vererything, since we have two hands, rule of threes.
// If you have a function that takes more than three arguments, 
// we need a container to put it in. Think of a bunch of Cadbury Mini Eggs.
// Hand me those one at a time. Now put them inside an Easter Egg. Much more efficient.
// All I have to do is open the one big egg to get what’s inside.
// In Rust, structs are our Easter Eggs. We use them as a container for types.
// Structs only have type declarations, no data. This is because we’re telling the compiler 
// what to expect so it knows how to allocate memory for how big or small our data will be.

struct easter_egg {
    owned_string: String,
    floaty: f64,
}
    
fn main() {
    let mut strings_and_float: easter_egg = easter_egg {
    owned_string: String::from(
    "The number cubed is: "),
    floaty: 3.33,
    };
    cube_floaty(&mut strings_and_float);
    println!("{}{}", strings_and_float.owned_string,
    strings_and_float.floaty);
}

fn cube_floaty(strings_and_float: &mut easter_egg) {
    strings_and_float.floaty = strings_and_float
    .floaty.powi(3);
}
