#![allow(unused)]

fn main() {
    // Scalar types
    // - single value
    // - building blocks for more complex types
    // Integers
    // Signed Integers
    // -(2 ** (n - 1)) to 2 ** (n - 1) - 1
    let i0: i8 = 1; // 8 bits
    let i1: i16 = 1; // 16 bits
    let i2: i32 = 1; // 32 bits
    let i3: i64 = 1; // 64 bits
    let i4: i128 = 1; // 128 bits
    let i5: isize = 1; // size of pointer

    // Unsigned Integers
    // 0 to 2 ** n - 1
    let u0: u8 = 1; // 8 bits
    let u1: u16 = 1; // 16 bits
    let u2: u32 = 1; // 32 bits
    let u3: u64 = 1; // 64 bits
    let u4: u128 = 1; // 128 bits
    let u5: usize = 1; // size of pointer

    // Floats
    let f0: f32 = 0.10; // 32 bits
    let f1: f64 = 210.10; // 64 bits

    // Boolean
    let b0: bool = true;

    // Characters
    let c0: char = 'a';

    // Type conversion
    let i: i32 = 42;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and Max values
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max:{max_i}");

    let min_u: u32 = u32::MIN;
    let max_u: u32 = u32::MAX;

    println!("u32 min:{min_u}");
    println!("u32 max:{max_u}");

    let min_f: f32 = f32::MIN;
    let max_f: f32 = f32::MAX;

    println!("f32 min:{min_f}");
    println!("f32 max:{max_f}");

    let min_c: char = char::MIN;
    let max_c: char = char::MAX;

    println!("char min:{min_c}");
    println!("char max:{max_c}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("u32 overflow: {u}");

    // checked_add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    // let u = u32::checked_add(3, 1);
    println!("checked_add: {:?}", u);

    // Wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}
