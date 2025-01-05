#![allow(unused)]

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl{h:u8, s:u8, l:u8},
}

fn main(){
    // Enum
    let color: Color = Color::Red;
    // Attributes - Debug and PartialEq
    // Option
    // Result


}