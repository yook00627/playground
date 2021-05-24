use std::io;

fn main() {
    let code = loop {
        println!(
            "Please type the nth fibonacci to generate\nIf you want to leave program type \"exit\""
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input");

        let input = input.trim();
        if input == "exit" {
            break 1;
        }

        let input: u128 = match input.parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Please type a natural number");
                continue;
            }
        };

        println!("You have converted {}", fibonacci(input));
    };

    println!("exit with code {}", code);
}

fn fibonacci(input: u128) -> u128 {
    if input == 1 {
        return 1;
    } else if input == 2 {
        return 1;
    } else {
        return fibonacci(input - 1) + fibonacci(input - 2);
    }
}
