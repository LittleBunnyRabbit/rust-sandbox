pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;

    if age >= 21 && check_id {
        println!("age >= 21");
    } else if age < 21 && check_id {
        println!("age < 21 && check_id");
    } else {
        println!("else");
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}