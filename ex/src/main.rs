
///Hello, This is my rust program!!! visit
/// https://www.rust-lang.org
///for info on rust itself.

// Import / Namespace
use std::{collections::HashMap, env, hash::Hash};
use regex::Regex;

// Debug
// ...These are a bit tricky to demonstrate, they can show up basically anywhere
// But they *are* always on thier own line, for what its worth
#[allow(clippy::approx_constant)]
// Also the colors here are actually VERY interchangable, so if one specific color clashes I can just remove it lol

// Constant
const PI: f64 = 3.1415926535;

#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

#[derive(Clone)]
pub struct Square {
    side_length: f64,
}

#[derive(PartialEq,PartialOrd)]
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
impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle {radius}
    }
}

const DEFAULT_NAME: &str = "Felix";

///General processes necessary for startup.
fn formalities() {
    let args: Vec<String> =
        env::args()
            .collect();
    // This is pretty intimidating looking, but this basically just translates to...
    // "A List of functions that accept a singl Vector (Vec) of Strings as an argument"
    let steps_in_formalities:
        [
            for<'a>
            fn(&'a
            std::vec::Vec
            <std::string::String>
            );
            4
        ]
        =
    [
        hello,
        greet,
        is_url,
        count
    ];

    let stepnames =
        HashMap::from([
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four")
        ]);

    let mut i = 0usize;

    for step in steps_in_formalities {
        step(&args);
        i += 1;
        println!(
            "just completed step {}!",
            stepnames.get
                (&i)
                .unwrap()
        );
    }
}

// Code that's ran *before* the program is executed.
// ...Is there an artist equivalent to this? Its like downloading a custom brush or smth lol
macro_rules! greet {
    ($name:expr) => {
        println!(
            "Hello, {}!",
            $name
        );
    };
}

fn hello(
    _args: &Vec<String>
) {
    let escaped_str =
        "Hello\t World";
    let bang: char = '!';
    let newline: char = '\n';

    // Yeah the {bang} bit could just be replaced by a "!"
    // But having text substitution with a single char demonstrates highlighting better
    print!(
        "my standard pulse is: {escaped_str}{bang}{bang}{bang}{newline}"
    );
}

fn greet(
    args: &Vec<String>
) {
    greet!(
        get_param(
            args,
            "name"
        )
        .unwrap_or(DEFAULT_NAME.to_string())
    );
}

fn get_param(
    args: &Vec<String>,
    to_get: &str
) ->
Option<String>
{

  return
    args
        .iter()
        .position(|s| *s=="-"+to_get)
        .and_then(|pos| args.get(pos + 1))
        .map(|name| name.to_string())
}

fn get_shape(
    args: &Vec<String>
) {
    let theshape =
        get_param(
            args,
            "shape"
        )
        .unwrap_or("".to_string());

    let regex =
        Regex::new
        ("(circle|square|triangle)")
        .unwrap();

    if let Some(capture)
        = regex.captures(&theshape)
    {
        println!(
            "The shape is... {}",
            capture
                .get(1)
                .unwrap()
                .as_str()
        );
    }
}

fn is_url(
    args: &Vec<String>
) {
    let url =
        get_param(
            args,
            "url"
        )
        .unwrap_or("".to_string());

    let re =
        Regex::new
        (r"^\w+://[\w.-]+$")
        .unwrap();

    if re.is_match(&url) {
        println!("is a url!");
    }
}

fn count(args: &Vec<String>) {
    let num =
        get_param(
            args,
            "num"
        )
        .and_then(|n| n.parse().ok())
        .unwrap_or(-1isize);

    if num < 0 {
        return;
    }

    println!("Counting up to {num}:");

    for i in 0..(num) {
        print!(" {i}");
    }

    println!("!\nJob done.");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // unused variables
    let int_val: i32 = 42;

    formalities();

    println!("Diff:\n- old value: {}\n+ new value: {}", 40, int_val);

    Ok(())
}
