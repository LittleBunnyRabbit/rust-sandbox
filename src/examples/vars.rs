pub fn run() {
    let name = "Brad";
    let mut age = 22;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 22);
    println!("{} is {}", my_name, my_age);
}