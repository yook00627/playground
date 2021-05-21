const MAX_NUMBER: u32 = 100_000_000;

// Just me playing around with Rust
fn main() {
    println!("Max number is: {}", MAX_NUMBER);

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let mut x = 5;

    x = x + 1;
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Needs to be a number!");
    println!("The value of x is: {}", guess);

    let tup: (i32, f64, u8) = (500000, 41.23, 122);
    println!("Print tuple: ({}, {}, {})", tup.0, tup.1, tup.2);

    let a: [i32; 2] = [1, 2];
    println!("Print array: [{}, {}]", a[0], a[1]);

    another_function();

    let x = six();

    println!("what is x? Its {}", x);

    println!("Add {} and {} is {}", x, 5, add(x, 5));

    if x > 3 {
        println!("{} is greater than 3", x);
    } else {
        println!("{} is lower than 3", x);
    }
}

fn another_function() {
    println!("another function running");

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x = {} y = {}", x, y);
}

fn six() -> i32 {
    6
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
