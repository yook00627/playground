fn main() {
    println!("Hello, world!");
    let a = 1234;
    let b = 4321;
    println!("{}", add(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
