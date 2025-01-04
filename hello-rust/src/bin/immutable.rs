#![allow(unused)]

fn main() {
    let x: i32 = -123;
    // x += 1; error thrown : cannot assign twice to immutable variable (Compilation error)

    let mut y: i32 = 123;
    y += 1;
    println!("value of y = {y}");

    let z = -123;
    // let w : () = 123; Way to find the datatype of the variable in Rust

    const NUM: u32 = 1;

    let mut x: i32 = -1;
    println!("value of x = {x}");

    let x: bool = true;
    println!("value of x = {x}");

    // x += 1; error thrown : cannot use `+=` on type `bool` (Compilation error)

    let v: Vec<_> = vec![1, 2, 3];
    println!("value of v = {:?}", v);
}
