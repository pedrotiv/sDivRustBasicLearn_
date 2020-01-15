use std::fmt; // Import `fmt`

#[derive(Debug)]
struct Complex{
    r: f64,
    i: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}

pub fn run(){
    let c = Complex{ r: 3.3, i:7.2};

    println!("Compare structure Complex:");
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}

// ACTIVITY:
//         After checking the output of the above example, 
//         use the Point2D struct as a guide to add a Complex struct to the example. 
//         When printed in the same way, the output should be:
//              Display: 3.3 + 7.2i
//              Debug: Complex { real: 3.3, imag: 7.2 }