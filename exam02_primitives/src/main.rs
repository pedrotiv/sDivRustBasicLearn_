use std::fmt;

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
// Implement Display for activity 1
impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"( {}, {} )\n( {}, {} )", self.0,self.1,self.2,self.3)           
    }
}


fn main() {
                            // PRIMITIVES //

    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;

                        // LITERAL AND OPERATORS //

        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
        // println!("1 - 2 = {}", 1u32 - 2); // Comile error, of course!!!!
    
        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);
    
        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);


                        // TUPLES //
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,-1i8,
        -2i16, -3i32, -4i64,0.1f32, 0.2f64,'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!("first elemet of inner tuple: {},",(tuple_of_tuples.0).0);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}",transpose(matrix));    
    }

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(mat: Matrix) -> Matrix{
    Matrix(mat.0, mat.2, mat.1, mat.2)
}


                        // NEXT: //

// Make the anoted TODO tasks:
// Do the activities below: 

// 1) Add the fmt::Display trait to the Matrix struct in the above example, 
// so that if you switch from printing the debug format {:?} to the display format {},
// you see the following output:
// ( 1.1 1.2 )
// ( 2.1 2.2 )




                        // SOMES RUST BITWISE OPERATORS //
                    
// Bitwise AND or &	Retorna 1 em cada posição de bit para à qual o bit correspondente de ambos eram 1s.
// Bitwise OR or |	Retorna 1 para cada posição de bit para à qual o correspondente de um ou ambos eram 1s.                    
// Bitwise XOR or ^ Retorna 1 para cada posição de bit para à qual o bit correspondente de um mas não ambos eram 1s.
// Bitwise NOT or ! Inverte os bits de seus operandos.
// Left shift	a << b	Jogam a  em representação binária b (< 32) bits à esquerda, mudando de zeros à diretia.
// Sign-propagating right shift	a >> b	Jogam a  em representação binária b (< 32) bits à direita, descartando bits que foram tornados off.

