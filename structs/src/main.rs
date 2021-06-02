//structs
#[derive(Debug)] // to get debug output 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn get_email(&self) -> String {
        self.email.clone()
    }
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn get_area(&self) -> u64 {
        self.width * self.height
    }

    fn increase_width(&mut self) {
        self.width += 10
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//tuple structs
struct Color(u8, u8, u8);

fn main() {
    //structs
    let user1 = User {
        username: String::from("Hz Y"),
        email: String::from("hazu@gmail.com"),
        sign_in_count: 0,
        active: false,
    };

    let user2 = make_user(String::from("ksjds@dandy.com"), String::from("kskvj"));

    let user3 = User {
        username: String::from("abc"),
        email: String::from("abc@gmail.com"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:#?}", user1); //pretty print

    println!("email: {}", user1.get_email());
    println!("username: {}", user1.username);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("active: {}", user1.active);

    println!("email: {}", user2.email);
    println!("username: {}", user2.username);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);

    println!("email: {}", user3.email);
    println!("username: {}", user3.username);
    println!("sign_in_count: {}", user3.sign_in_count);
    println!("active: {}", user3.active);

    let mut rect1 = Rectangle {
        width: 50,
        height:22,
    };

    println!("Area: {}", rect1.get_area());
    rect1.increase_width();
    println!("Area: {}", rect1.get_area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can it hold? {}", if rect1.can_hold(&rect2) {"Yes"} else {"No"});
    println!("Can it hold? {}", if rect1.can_hold(&rect3) {"Yes"} else {"No"});

    //tuple structs
    let color1 = Color(0, 245, 222);

    println!("R: {}", color1.0);
    println!("G: {}", color1.1);
    println!("B: {}", color1.2);
}

fn make_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}