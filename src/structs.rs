// Structs - used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        // @NOTE: &self is a reference to the object itself (like *this* in other langs)
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        // @NOTE: &mut self is a reference to the object itself with mutable permissions enabled
        self.last_name = last.to_string();
    }

    // Name To Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    let mut c2 = Color2(255, 0, 0);

    c.red = 200;
    c2.0 = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());
}