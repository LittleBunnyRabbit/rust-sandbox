pub fn run() {
    greeting("Hello", "Jane");
    
    let get_sum: i32 = add(5, 5);
    println!("{}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_nums(10, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}