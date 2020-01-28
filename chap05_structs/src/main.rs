// Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data.
// We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (it include &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }


    // Method (it include &self)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (it does'n include &self)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}



