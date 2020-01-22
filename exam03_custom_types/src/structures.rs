// There are three types of structures ("structs") that can be created using the struct keyword:

// Tuple structs, which are, basically, named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics.

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // note bellow the reason to use a constructer...kkkk
    let rect: Rectangle = Rectangle {top_left: Point {x: 0.0, y: 30.0}, bottom_right: Point{x:10.0, y:0.0}};

    impl Rectangle{
        fn area(&self) -> f32{
            let height = self.top_left.y - self.bottom_right.y;
            let width = self.top_left.x - self.bottom_right.x;
            (height * width).abs()
        }

        fn square(origin: Point, size: f32) -> Rectangle{
            Rectangle{top_left: {Point{x: origin.x, y: origin.x+size}}, 
                      bottom_right: {Point{x: origin.y+size, y: origin.x}}}
        }

    }
     

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("The area of rectangle is {:.2}",rect.area());

    let sqr = Rectangle::square(Point{x: 0.0, y:0.0},30.0);

    println!("The square: {:?}", sqr);

    println!("The area of rectangle is {:.2}",sqr.area());
    

}

                        // NEXT //
// Activity: 

// Add a function rect_area which calculates the area of a rectangle. OK!!!!!!!!!!!!!!!!

// Add a function square which takes a Point and a f32 as arguments, 
// and returns a Rectangle with its lower left corner on the point, 
// and a width and height corresponding to the f32.  OK!!!!!!!!!!!!!!!!!!!!!!!!!!!
