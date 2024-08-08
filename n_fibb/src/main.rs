use std::io;

fn fib(n :  u32) -> u32{
    
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    
    return fib(n-1) + fib(n-2);
}
fn main() {
    println!("Nth Fibonacci Number");
    loop {
        println!("Enter a number n , where n>=0 ");
        let mut input = String::new();
        io::stdin().read_line(& mut input).expect("Failed to read line");
        let input = match input.trim().parse::<u32>() { 
            Ok(num) => num,
            Err(e) => {
                println!("{}",e);
                continue;
            }
        };
        println!("{}th fibonacci = {}",input,fib(input));
    }
}
