use std::io;

pub fn run() {
    let mut in_num = String::new();
    loop {
        let mut fib: i64 = 0;
        let mut ant: i64 = 0;
        println!("\nTo obtain a fibonacci series, input a positive integer number: ");
        io::stdin()
            .read_line(&mut in_num)
            .expect("Failed to read line");
        let in_num: i64 = match in_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        print!("Fibonacci Serie: ");
        for i in 1..in_num+1 {
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
*       Create a code to exit
*       Implement the fibonacci series
*       The loop doesn't work afer firt interation
*/

