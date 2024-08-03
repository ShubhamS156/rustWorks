use std::io;

fn main() {
    println!("Enter temperature to convert.");
    println!("Example: 32c or 32C or 100f or 100F");

    loop {
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        //we have the temperature , now need to extract unit from it.
        //before extracting we have to trim the input first.
        let temperature = temperature.trim();
        let unit : char = temperature.chars().last().unwrap();
        let temperature = &temperature[..temperature.len()-1];
        let temperature = match temperature.parse::<f64>(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Should [number][C/c | F/f]");
                continue;
            },
        };
        
        match unit {
            'C' | 'c' => {
                println!("Input = {temperature}Celsius");
                println!("Output = {}Fahrenheit",(temperature * 9f64 / 5f64 )+ 32f64);
            },
            'F' | 'f' => {
                println!("Input = {temperature}Fahrenheit");
                println!("Output = {}Celsius",(temperature-32f64)*5f64/9f64);
            },
            
            _ => {
                println!("Invalid unit");
                continue;
            }
        };

    }

}