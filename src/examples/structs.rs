struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct ColorTouple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_touple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let ct = ColorTouple(255, 0, 0);
    println!("ColorTouple: {} {} {}", ct.0, ct.1, ct.2);


    let mut p = Person::new("John", "Doe");
    println!("{}", p.full_name());
    p.set_last_name("Banana");
    println!("{}", p.full_name());

    println!("{:?}", p.to_touple());
}