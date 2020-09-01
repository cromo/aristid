use std::rc::Rc;

fn main() {
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
