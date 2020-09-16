use std::rc::Rc;

fn main() {
    let algae = aristid::LSystem::new(
        vec![Algae::A],
        vec![
            Rc::new(|s| match s {
                Algae::A => Some(vec![Algae::A, Algae::B]),
                _ => None,
            }),
            Rc::new(|s| match s {
                Algae::B => Some(vec![Algae::A]),
                _ => None,
            }),
        ],
    );
    println!(
        "Axiom: {:?}, P1: {:?}, P2: {:?}, P3: {:?}",
        algae.symbols,
        algae.apply().symbols,
        algae.apply().apply().symbols,
        algae.apply().apply().apply().symbols
    );
    println!("what is going on up there?");
}

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}
