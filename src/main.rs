fn main() {
    let algae = aristid::LSystem::new(
        vec![Algae::A],
        vec![
            &|s| match s {
                Algae::A => Some(vec![Algae::A, Algae::B]),
                _ => None,
            },
            &|s| match s {
                Algae::B => Some(vec![Algae::A]),
                _ => None,
            },
        ],
    );
    println!(
        "Axiom: {:?}, P1: {:?}, P2: {:?}, P3: {:?}",
        algae.symbols,
        algae.apply().symbols,
        algae.apply().apply().symbols,
        algae.apply().apply().apply().symbols
    );
}

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}
