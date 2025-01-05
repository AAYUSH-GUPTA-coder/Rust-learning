#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Array - fixed length, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    // change the value of the array
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[0] = 10;
    println!("arr[0] = {}", arr[0]);

    // Set the each element value to Zero
    let arr: [u32; 10] = [0; 10]; // initialize the array with 0
    println! {"arr value = {:?}", arr};

    // Slice - length not known at compile time
    let nums: [i32; 10] = [1, -2, 3, 4, 5, -6, 7, 8, -9, 10];

    // First 3 elements
    let s = &nums[0..3]; // 0, 1, 2 // from 0 element upto 3rd element but not include 3rd element
                         // let s = &nums[..3]; // 0, 1, 2 // You can also do this way, give the same result
    println!("s = {:?}", s);

    // Middle 4 elements
    let s = &nums[3..7]; // 3, 4, 5, 6
    println!("s = {:?}", s);

    // Last 3 elements
    let s = &nums[7..10]; // 7, 8, 9
    let s = &nums[7..]; // 7, 8, 9 // You can also do this way, give the same result
    println!("s = {:?}", s);

    // All elements
    println!("All elements = {:?}", &nums[..]);
}
