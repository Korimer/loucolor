//! This module demonstrates many Rust concepts in one snippet

// Import / Namespace
use std::collections::HashMap;
use regex::Regex;

// Constant
const PI: f64 = 3.1415926535;

// Type / Modifier
pub struct Circle {
    // Member Variable / Property
    radius: f64,
}

impl Circle {
    // Function with Parameter Variable
    pub fn new(radius: f64) -> Circle {
        return Circle { radius };
    }

    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// Macro
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Variable
    let name = "Rustacean";
    let int_val: i32 = 42;
    let float_val: f64 = 3.14;
    let escaped_str = "Hello\nWorld\t!";
    let ch: char = 'R';
    let special = "@#$%^&*";
    let url = "https://www.rust-lang.org";
    let path = "/usr/local/bin";

    println!("Debug info: {:?}", (name, int_val, float_val, escaped_str, ch));

    greet!(name);

    for i in 0.. 3 {
        println!("Repeat count: {}", i);
    }

    if int_val > 40 {
        println!("int_val is greater than 40");
    } else {
        println!("int_val is 40 or less");
    }

    let re = Regex::new(r"^\w+://[\w.-]+$")?;
    if re.is_match(url) {
        println!("Valid URL detected!");
    }

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("one".to_string(), 1);

    println!("*** Highlighted: {} ***", name);
    println!("--- Underlined: {} ---", name);

    let array = [1, 2, 3];
    let tuple = (1, "two");
    let sum = array[0] + int_val;

    println!("Diff:\n- old value: {}\n+ new value: {}", 40, int_val);

    return Ok(());
}
