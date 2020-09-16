use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);

    numbers[2] = 4;
    println!("{:?}", numbers);
    println!("Len: {}", numbers.len());
    println!("bytes: {}", mem::size_of_val(&numbers));
    
    let slice_full: &[i32] = &numbers;
    println!("{:?}", slice_full);

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}