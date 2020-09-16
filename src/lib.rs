use std::rc::Rc;

pub struct LSystem<Alphabet> {
  pub symbols: Vec<Alphabet>,
  pub productions: Vec<Rc<dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>>,
}

impl<Alphabet: Clone + Copy> LSystem<Alphabet> {
  pub fn new(
    axiom: Vec<Alphabet>,
    productions: Vec<Rc<dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>>,
  ) -> LSystem<Alphabet> {
    LSystem::<Alphabet> {
      symbols: axiom,
      productions,
    }
  }

  pub fn apply(&self) -> LSystem<Alphabet> {
    LSystem::<Alphabet> {
      symbols: self
        .symbols
        .iter()
        .flat_map(|symbol| step(symbol, &self.productions))
        .collect(),
      productions: self.productions.clone(),
    }
  }
}

pub fn step<Alphabet: Copy>(
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
