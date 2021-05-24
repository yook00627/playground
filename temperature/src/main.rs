use std::io;

fn main() {
    let code = loop {
        println!(
            "Please type a temperature in Celsius you want to convert to Fahrenheit
If you want to leave program type \"exit\""
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input");

        let input = input.trim();
        if input == "exit" {
            break 1;
        }
        let input: f32 = match input.parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        let converted: f32 = input / 5.0 * 9.0 + 32.0; // conversion equation

        println!("You have converted {:.2}\u{00B0}C to {:.2}\u{00B0}F", input, converted);
    };

    println!("exit with code {}", code);
}
