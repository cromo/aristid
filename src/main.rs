// use std::option::*;

fn main() {
    println!("Hello, world!");
    let axiom = "A";
    println!(
        "Axiom: {}, P1: {}, P2: {}, P3: {}",
        axiom,
        step(axiom),
        step(&step(axiom)),
        step(&step(&step(axiom)))
    );
}

fn grow(s: &str) -> Option<&str> {
    if s == "A" {
        Some("AB")
    } else {
        None
    }
}

fn convert(s: &str) -> Option<&str> {
    if s == "B" {
        Some("A")
    } else {
        None
    }
}

fn apply_productions(s: &str) -> &str {
    grow(s).or(convert(s)).unwrap_or(s)
}

fn step(s: &str) -> String {
    s.split("")
        .map(apply_productions)
        .fold(String::new(), |a, b| a + b)
}

// G = (V, omega, P) where
// - V is the alphabet
// - omega is the axiom
// - P is the production rules

// So the main things to know from this implementation are:
// 1. The axiom and intermediate sequences should be a collection of symbols
// 2. The productions should return an optional list of symbols
// 3. The `apply_productions` should take a collection of productions to apply
// 4. The above combine to make the main state needed to track the starting
//    point or any state thereafter is a collection of symbols and a collection
//    of productions.
// 5. The alphabet can be specified as a type for compile-time specified
//    L-systems.
