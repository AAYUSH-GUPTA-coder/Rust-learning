#![allow(unused)]

#[derive(Debug)] // This is a derive attribute. This will tell the rust to autogenerate a code so that struct can be printed nicely

struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    // Positional arguments
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };

    println!("{:?}", lang); // print the struct
    println!("{:#?}", lang); // print the struct with line breaks
}
