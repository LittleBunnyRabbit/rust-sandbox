/*
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean: bool
 * Character: char
 * Touples
 * Arrays
 */

pub fn run() {
    let y: i64 = 4654654654654;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1: char = 'a'; // Unicode
    let face = '\u{1F600}';

    println!("{:?}", (is_active, y, is_greater, a1, face));
}