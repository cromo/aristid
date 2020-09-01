use std::rc::Rc;

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
    let algae = LSystem::new(
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
}

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}

struct LSystem<Alphabet> {
    symbols: Vec<Alphabet>,
    productions: Vec<Rc<dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>>,
}

impl<Alphabet: Clone + Copy> LSystem<Alphabet> {
    fn new(
        axiom: Vec<Alphabet>,
        productions: Vec<Rc<dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>>,
    ) -> LSystem<Alphabet> {
        LSystem::<Alphabet> {
            symbols: axiom,
            productions,
        }
    }

    fn apply(&self) -> LSystem<Alphabet> {
        LSystem::<Alphabet> {
            symbols: self
                .symbols
                .iter()
                .flat_map(|symbol| step2(symbol, &self.productions))
                .collect(),
            productions: self.productions.clone(),
        }
    }
}

fn step2<Alphabet: Copy>(
    symbol: &Alphabet,
    productions: &Vec<Rc<dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>>,
) -> Vec<Alphabet> {
    productions
        .iter()
        .map(|p| p(symbol))
        .find(|res| res.is_some())
        .unwrap_or_else(|| Some(vec![*symbol]))
        .unwrap()
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
