use std::io;

fn main() {
    println!("Hello, these are some exercises to practice rust!");

    loop {
        println!("Choose a exercise:");
        println!("1) Convert Temperatures between Fahrenheit and Celsius");
        println!("2) Generate the nth Fibonacci number");
        println!("3) Print the lyrics to \"The Twelve Days of Christmas\"");
        println!("Or type 0 to exit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("oh no, couldn't read your input :(");

        let input: u32 = input.trim().parse().expect("That wasn't a number!");

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
	println!("not implemented yet!");
}

fn fibonacci() {
	println!("not implemented yet!");
}

fn twelve_days(){
	println!("not implemented yet!");
}

