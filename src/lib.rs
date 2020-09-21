pub struct LSystem<'a, Alphabet> {
  pub symbols: Vec<Alphabet>,
  pub productions: Vec<&'a dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>,
}

impl<Alphabet: Copy> LSystem<'_, Alphabet> {
  pub fn new(
    axiom: Vec<Alphabet>,
    productions: Vec<&dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>>,
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
    .flatten()
    .unwrap_or_else(|| vec![*symbol])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
  enum Alphabet {
    A,
    B,
    C,
  }

  #[test]
  fn no_applicable_rules_leaves_passes_original_symbol() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![&|s| match s {
        Alphabet::B => Some(vec![Alphabet::C]),
        _ => None,
      }],
    );
    assert_eq!(vec![Alphabet::A], system.apply().symbols);
  }

  #[test]
  fn matching_rule_is_applied() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![&|s| match s {
        Alphabet::A => Some(vec![Alphabet::B]),
        _ => None,
      }],
    );
    assert_eq!(vec![Alphabet::B], system.apply().symbols);
  }

  #[test]
  fn first_matching_rule_is_applied() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![
        &|s| match s {
          Alphabet::A => Some(vec![Alphabet::B]),
          _ => None,
        },
        &|s| match s {
          Alphabet::A => Some(vec![Alphabet::C]),
          _ => None,
        },
      ],
    );
    assert_eq!(vec![Alphabet::B], system.apply().symbols);
  }

  #[test]
  fn symbols_can_be_replaced_with_multiple_symbols() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![&|s| match s {
        Alphabet::A => Some(vec![Alphabet::B, Alphabet::C]),
        _ => None,
      }],
    );
    assert_eq!(vec![Alphabet::B, Alphabet::C], system.apply().symbols);
  }

  #[test]
  fn symbols_can_be_relpaced_with_no_symbols() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![&|s| match s {
        Alphabet::A => Some(vec![]),
        _ => None,
      }],
    );
    assert_eq!(Vec::<Alphabet>::new(), system.apply().symbols);
  }

  #[test]
  fn multiple_replacements_happen_in_parallel() {
    let system = LSystem::new(
      vec![Alphabet::A, Alphabet::B, Alphabet::A],
      vec![
        &|s| match s {
          Alphabet::A => Some(vec![Alphabet::B]),
          _ => None,
        },
        &|s| match s {
          Alphabet::B => Some(vec![Alphabet::C]),
          _ => None,
        },
      ],
    );
    assert_eq!(
      vec![Alphabet::B, Alphabet::C, Alphabet::B],
      system.apply().symbols
    );
  }
}
