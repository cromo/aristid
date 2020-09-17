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

    // let rules: Vec<&dyn Fn(&Algae) -> Option<Vec<Algae>>> = vec![
    //     &|x| match x {
    //         Algae::A => Some(vec![Algae::A, Algae::B]),
    //         _ => None,
    //     },
    //     &|x| match x {
    //         Algae::B => Some(vec![Algae::A]),
    //         _ => None,
    //     },
    // ];
    // for rule in &rules {
    //     println!("{:?} -> {:?}", Algae::A, rule(&Algae::A));
    //     println!("{:?} -> {:?}", Algae::B, rule(&Algae::B));
    // }
    let new_algae = LSystem::new(
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
    println!("new system: {:?}", new_algae.symbols);
    println!("new system P1: {:?}", new_algae.apply().symbols);
}

struct LSystem<'a, Alphabet> {
    symbols: Vec<Alphabet>,
    productions: Vec<&'a dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>,
}

impl<Alphabet: Copy> LSystem<'_, Alphabet> {
    fn new(
        axiom: Vec<Alphabet>,
        productions: Vec<&dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>,
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
                .flat_map(|s| step(s, &self.productions))
                .collect(),
            productions: self.productions.clone(),
        }
    }
}

fn step<Alphabet: Copy>(
    symbol: &Alphabet,
    productions: &Vec<&dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>,
) -> Vec<Alphabet> {
    productions
        .iter()
        .map(|p| p(symbol))
        .find(|res| res.is_some())
        .unwrap_or_else(|| Some(vec![*symbol]))
        .unwrap()
}

// fn f1(x: &Algae) -> Option<Vec<Algae>> {
//     match x {
//         Algae::A => Some(vec![Algae::A, Algae::B]),
//         _ => None,
//     }
// }

// fn f2(x: &Algae) -> Option<Vec<Algae>> {
//     match x {
//         Algae::B => Some(vec![Algae::A]),
//         _ => None,
//     }
// }

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}
