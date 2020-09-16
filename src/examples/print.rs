pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Basic formatting
    println!("Number: {}, String: {}", 1, "Hi");

    // Positional Arguments
    println!("{2}, {1}, {0}", "First", "Second", "Third");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Jon", activity = "Baseball");

    // Placeholder traits
    println!("Binard: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}