// length is fixed

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    numbers[2] = 4;
    println!("{:?}", numbers);
    println!("Len: {}", numbers.len());
    println!("bytes: {}", mem::size_of_val(&numbers));
    
    let slice_full: &[i32] = &numbers;
    println!("{:?}", slice_full);

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}