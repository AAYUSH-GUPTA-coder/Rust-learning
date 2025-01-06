#![allow(unused)]
#[derive(Debug, PartialEq)]

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    // Enum
    // explicitly decleared
    let color: Color = Color::Red;
    // implicitly decleared
    let color = Color::Green;
    let color = Color::Rgba(0, 0, 255, 0.1);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl { h: 2, s: 1, l: 100 };
    // Attributes - Debug and PartialEq
    println!("{:?}", color);
    // ParticalEq
    // compare the value of 2 enum types
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // Most common ENUM used in rust
    // 1. Option : Some(23) | None
    // Option is a enum that can take on two values either Some() | None. Inside this parentheses of Some() you'll have some kind of value for example if u32, then it will be Some(5) or if string then Some("Hello")
    let x : Option<i32> = None;
    let x : Option<i32> = Some(-5);
    println!("Option: {:?}", x);

    // 2. Result : Ok(23) | Err("error message")
    // It takes two types of values inside the enum. one is the datatype of Ok() and other is the datatype of the Err().
    // Result is a type of enum, that is kind of like an option and it represents whether something was successful or not. In case of successful it will return Ok() | Err(). Inside this parentheses of Ok() it will return the result otherwise it will return the Err() error with message.
    let res: Result<i32, String> = Ok(200);
    println!("Result: {:?}", res);
    let res: Result<i32, String> = Err("div by 0".to_string());
    println!("Result: {:?}", res);

}
