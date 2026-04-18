use std::io;

fn main() {
    println!("=== Fahrenheit <-> Celsius conversion ===");
    loop {
        /* Ask user for type of conversion */
        println!("Please type one of the following options. Usage:");
        println!("  F = Convert a temperature from Fahrenheit to Celsius");
        println!("  C = Convert a temperature from Celsius to Fahrenheit");
        println!("  Q = Quit the program");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        /* Verify input */
        let result_unit: &str;
        match input.as_str().trim() {
            "F" => 
            {
                println!("Selected: Fahrenheit to Celsius conversion.");
                result_unit = "C";
            }
            "C" => {
                println!("Selected: Celsius to Fahrenheit conversion.");
                result_unit = "F";
            }
            "Q" => break,
             _  => {
                println!("Error: Please type a valid option.");
                continue;
            },
        };

        /* Ask user for temperature */
        println!("Please type a temperature value in {}°.", input.trim());

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please type a valid temperature value.");
                continue;
            },
        };

        /* Verify and calculate temperature */
        const ABSOLUTE_ZERO_FAHRENHEIT: f64 = -459.670;
        const ABSOLUTE_ZERO_CELSIUS: f64 = -273.150;

        let result: f64;
        match input.as_str().trim() {
            "F" => {
                if temp < ABSOLUTE_ZERO_FAHRENHEIT
                {
                    println!("Error: Please type a valid temperature value.");
                    continue;
                }
                else
                {
                    result = (5.0/9.0)*(temp - 32.0);
                }
            },
            "C" => { 
                if temp < ABSOLUTE_ZERO_CELSIUS
                {
                    println!("Error: Please type a valid temperature value.");
                    continue;
                }
                else
                {
                    result = (9.0/5.0)*temp + 32.0;
                }
            },
            _ => todo!(),
        };
        println!("The temperature result is {result:.3}{}°", result_unit);
    }
    println!("Quitting program.");
}
