use std::io;

pub fn run() {
    // Convert temperatures between Fahrenheit and Celsius
    
    loop {        
        let mut in_temp = String::new();
        println!("Input temperature to convert to Celsius and Fahrenheit or 's' to stop: ");
        io::stdin()
            .read_line(&mut in_temp)
            .expect("Failed to read line");     
        if in_temp.trim() == "s" { break; }   
        let in_temp: f64 = match in_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };        
        {
            let out_c: f64 = (in_temp - 32.0) / 1.8;
            let out_f: f64 = 1.8 * in_temp + 32.0;
            println!("The temperature {} F is {} C", in_temp, out_c);
            println!("The temperature {} C is {} F", in_temp, out_f);
        }
    }
}
/* Next: 
*       
*/