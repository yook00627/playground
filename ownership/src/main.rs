fn main() {
    let mut s = String::from("Hello");

    s.push_str(" World");
    add_exclamation(&mut s);

    println!("{}", get_length(&s));
    println!("{}", s);

    {
        let s1 = &mut s;
        println!("{}", s1);
    }

    let s2 = &s;
    let s3 = &s;

    println!("{}", s2);
    println!("{}", s3);

    println!("{}", first_word(&s[..]));
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn add_exclamation(s: &mut String) {
    s.push_str("!")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}