use std::io;

pub fn run() {
    
    loop {
        let mut in_num = String::new();
        let mut fib: i64 = 0;
        let mut ant: i64 = 0;
        println!("\nTo obtain a fibonacci series, input a positive integer number or 's' to exit: ");
        io::stdin()
            .read_line(&mut in_num)
            .expect("Failed to read line");
        if in_num.trim() == "s" { break; }
        let num_in: i64 = match in_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };        
        print!("Fibonacci Serie: ");
        for i in 1..num_in+1 {
            if i == 1 {
                fib = 1;
                ant = 0;
            } else{
                fib += ant;
                ant = fib - ant;
            }            
            print!("{}:{} ", i, fib);
        }
    }
}
/*
* Next:
*       
*/

