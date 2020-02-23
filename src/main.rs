use std::io;

fn main() {
    println!("Hello, these are some exercises to practice rust!");

    loop {
        println!("Choose a exercise:");
        println!("1) Convert Temperatures between Fahrenheit and Celsius");
        println!("2) Generate the nth Fibonacci number");
        println!("3) Print the lyrics to \"The Twelve Days of Christmas\"");
        println!("Or type 0 to exit\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("oh no, couldn't read your input :(");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 10,
        };

        match input {
            1 => temperature_converter(),
            2 => fibonacci(),
            3 => twelve_days(),
            0 => break,
            _ => println!("that wasn't an option! Try again!"),
        }
    }
}

fn temperature_converter() {
    loop {
        println!(
            "Type a temperature and we'll convert it to the other standard for you: (XXC/XXF)"
        );
        println!("If you don't want to do this type 'exit' to return to the menu\n");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("oh no, couldn't read your input :(");
        input = input.trim().to_string();

        if input == "exit" {
            break;
        }

        let mut temp_scale = match input.pop() {
            Some(scale) => scale,
            None => continue,
        };
        temp_scale.make_ascii_lowercase();

        let input: f32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you didn't input a valid number! try again\n");
                continue;
            }
        };

        match temp_scale {
            'f' => {
                println!("That's {}{}", fahrenheit_to_celsius(input), 'C');
            }
            'c' => {
                println!("That's {}{}", celsius_to_fahrenheit(input), 'F');
            }
            _ => {
                println!("{} is not a valid temperature scale!", temp_scale);
            }
        };
        println!("");
    }
}

fn fahrenheit_to_celsius(val: f32) -> f32 {
    return (val - 32.0) * (5.0 / 9.0);
}

fn celsius_to_fahrenheit(val: f32) -> f32 {
    return val * (9.0 / 5.0) + 32.0;
}

fn fibonacci() {
    println!("not implemented yet!");
}

fn twelve_days() {
    println!("not implemented yet!");
}
