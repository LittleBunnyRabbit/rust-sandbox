// str = fixed length
// String = Growable, heap-allocated

pub fn run() {
    let hello = "Hello ";
    let mut hello_growable = String::from(hello);

    println!("{}, {}", hello, hello_growable);
    println!("Length: {}, Length: {}", hello.len(), hello_growable.len());

    hello_growable.push('W');

    println!("{}", hello_growable);

    hello_growable.push_str("orld");

    println!("{}", hello_growable);

    // Capacity in bytes
    println!("Capacity: {}", hello_growable.capacity());

    println!("Is empty: {}", hello_growable.is_empty());

    println!("Contains 'World': {}", hello_growable.contains("World"));

    println!("Replace 'World': {}", hello_growable.replace("World", "There"));

    // Loop trough string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capa
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}