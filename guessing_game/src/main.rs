use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("The secret num is {} ",secret_num);
    println!("Please input your guess");
    loop {
        
    let mut guess = String::new();
    let num;
    num = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse(){
        Ok(a) => a,
        Err(_) => continue,
    };
    
    println!("Read {} bytes",num);
    println!("You guessed: {}",guess);

    match guess.cmp(&secret_num){
        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => {
            println!("You WIN");
            break;
        }
        Ordering::Greater => println!("Too Large")
    }
}
}

