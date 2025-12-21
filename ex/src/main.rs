//Hello, This is my rust program!!! visit
// https://www.rust-lang.org
//for info on rust itself.

// Import / Namespace
use std::{collections::HashMap, env};
use regex::Regex;

// Constant
const PI: f64 = 3.1415926535;

/// Pub is a Modifier, struct is a Type
pub struct Circle {
    radius: f64,
}
pub struct Square {
    side_length: f64,
}
pub struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}
pub enum Shape {
    Circle(Circle),
    Square(Square),
    Triangle(Triangle),
}
impl SimpleShape for Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle(c) => c.area(),
            Self::Square(s) => s.area(),
            Self::Triangle(t) => t.area(),
        }
    }
}

trait SimpleShape {
    fn area(&self) -> f64;
}

impl SimpleShape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl SimpleShape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

impl SimpleShape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2f64;
        (s * (s-self.side1) * (s-self.side2) * (s-self.side3)).sqrt()
    }
    
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle {radius}
    }
}
impl Square {
    pub fn new(side_length: f64) -> Self {
        Square { side_length }
    }
}
impl Triangle {
    pub fn new(side1: f64, side2: f64, side3: f64) -> Self {
        Triangle { side1, side2, side3 }
    }
}


const DEFAULT_NAME: &str = "Felix";

fn formalities() {
    let args: Vec<String> = env::args().collect();
    hello();
    greet(args);
}

// Macro
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn hello() {
    let escaped_str = "Hello\t World";
    let bang: char = '!';
    let newline: char = '\n';
    print!("my standard pulse is: {escaped_str}{bang}{bang}{bang}{newline}")
}

fn greet(args: Vec<String>) {
    greet!(
        get_param(args, "name")
            .unwrap_or(DEFAULT_NAME.to_string())
    );
}

fn get_param(args: Vec<String>,to_get: &str) -> Option<String> {
    args
        .iter()
        .position(|s| *s == "-".to_string() + to_get)
        .and_then(|pos| args.get(pos))
        .map(|name| name.to_string())
}

fn get_shape(args: Vec<String>) -> Option<String> {
    let theshape = get_param(args, "shape").unwrap_or("".to_string());
    let regex = Regex::new("(circle|square|triangle)").unwrap();
    for (_, [shapetype]) in regex.captures(&theshape).map(|c| c.extract()) {
        return Some(shapetype.to_string());
    }
    return None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Variable
    let int_val: i32 = 42;
    let float_val: f64 = 3.14;
    let path = "/usr/local/bin";

    for i in 0.. 3 {
        println!("Repeat count: {i}");
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
