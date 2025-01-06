#![allow(unused)]

// String and &str
fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8. Vector are link array that can grow and shrink in size
    // &str = slice (string slice) of u8 ( &[u8] ) valid UTF-8.

    // when to use String vs &str
    // String -> mutate or data needs to be owned
    // &str -> read only

    // String
    let msg: String = String::from("Hello Rust 🦀");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");

    // str - string slice
    // &str
    // - usually used str with reference (borrowed)
    // - immutable

    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg[..5]; // first 5 characters
    let len: usize = s.len();
    println!("s: {s}");
    println!("len: {len}");

    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of the binary
    // - immutable because hard coded inside the binary
    let hello: &str = "Hello Rust 🦀";

    // Multiline string literal using &str
    let s: &str = r#"
        {
            "a":1,
            "b":{"c": 2},
            "d":3
        }
    "#;
    println!("s: {s}");

    // Deref coercion
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg;

    // Add &str to String
    let mut msg: String = "Hello Rust".to_string();
    msg += " 🦀";
    println!("msg: {msg}");

    msg.push_str(" 🦀");
    println!("msg: {msg}");

    // String interpolation
    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    println!("msg: {msg}");
}
