use std::iter;

pub struct LSystem<'a, Alphabet> {
  pub symbols: Vec<Alphabet>,
  pub productions: Vec<Production<'a, Alphabet>>,
}

#[derive(Clone, Copy)]
pub enum Production<'a, Alphabet> {
  ContextFree(&'a dyn Fn(&Alphabet) -> Option<Vec<Alphabet>>),
  PriorContext(&'a dyn Fn(&Alphabet, &Alphabet) -> Option<Vec<Alphabet>>),
  FollowingContext(&'a dyn Fn(&Alphabet, &Alphabet) -> Option<Vec<Alphabet>>),
  SurroundingContext(&'a dyn Fn(&Alphabet, &Alphabet, &Alphabet) -> Option<Vec<Alphabet>>),
}

impl<Alphabet: Copy> LSystem<'_, Alphabet> {
  pub fn new(axiom: Vec<Alphabet>, productions: Vec<Production<Alphabet>>) -> LSystem<Alphabet> {
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
        .zip(
          iter::once(None).chain(self.symbols.iter().map(Some)).zip(
            self
              .symbols
              .iter()
              .skip(1)
              .map(Some)
              .chain(iter::once(None)),
          ),
        )
        .flat_map(|(s, (predecessor, successor))| {
          step(predecessor, s, successor, &self.productions)
        })
        .collect(),
      productions: self.productions.clone(),
    }
  }
}

fn step<Alphabet: Copy>(
  predecessor: Option<&Alphabet>,
  symbol: &Alphabet,
  successor: Option<&Alphabet>,
  productions: &Vec<Production<Alphabet>>,
) -> Vec<Alphabet> {
  productions
    .iter()
    .map(|p| match (p, predecessor, symbol, successor) {
      (Production::ContextFree(f), _, s, _) => f(s),
      (Production::PriorContext(f), Some(pre), s, _) => f(pre, s),
      (Production::FollowingContext(f), _, s, Some(post)) => f(s, post),
      (Production::SurroundingContext(f), Some(pre), s, Some(post)) => f(pre, s, post),
      _ => None,
    })
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
  fn no_matching_rules_produces_same_symbol() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![Production::ContextFree(&|s| match s {
        Alphabet::B => Some(vec![Alphabet::C]),
        _ => None,
      })],
    );
    assert_eq!(vec![Alphabet::A], system.apply().symbols);
  }

  #[test]
  fn matching_rule_is_applied() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![Production::ContextFree(&|s| match s {
        Alphabet::A => Some(vec![Alphabet::B]),
        _ => None,
      })],
    );
    assert_eq!(vec![Alphabet::B], system.apply().symbols);
  }

  #[test]
  fn first_matching_rule_is_applied() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![
        Production::ContextFree(&|s| match s {
          Alphabet::A => Some(vec![Alphabet::B]),
          _ => None,
        }),
        Production::ContextFree(&|s| match s {
          Alphabet::A => Some(vec![Alphabet::C]),
          _ => None,
        }),
      ],
    );
    assert_eq!(vec![Alphabet::B], system.apply().symbols);
  }

  #[test]
  fn symbols_can_be_replaced_with_multiple_symbols() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![Production::ContextFree(&|s| match s {
        Alphabet::A => Some(vec![Alphabet::B, Alphabet::C]),
        _ => None,
      })],
    );
    assert_eq!(vec![Alphabet::B, Alphabet::C], system.apply().symbols);
  }

  #[test]
  fn symbols_can_be_relpaced_with_no_symbols() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![Production::ContextFree(&|s| match s {
        Alphabet::A => Some(vec![]),
        _ => None,
      })],
    );
    assert_eq!(Vec::<Alphabet>::new(), system.apply().symbols);
  }

  #[test]
  fn multiple_replacements_happen_in_parallel() {
    let system = LSystem::new(
      vec![Alphabet::A, Alphabet::B, Alphabet::A],
      vec![
        Production::ContextFree(&|s| match s {
          Alphabet::A => Some(vec![Alphabet::B]),
          _ => None,
        }),
        Production::ContextFree(&|s| match s {
          Alphabet::B => Some(vec![Alphabet::C]),
          _ => None,
        }),
      ],
    );
    assert_eq!(
      vec![Alphabet::B, Alphabet::C, Alphabet::B],
      system.apply().symbols
    );
  }

  #[test]
  fn when_a_contextual_rule_doesnt_match_use_the_identity_rule() {
    let system = LSystem::new(
      vec![Alphabet::A],
      vec![Production::SurroundingContext(
        &|pre, s, post| match (pre, s, post) {
          (Alphabet::B, Alphabet::A, Alphabet::C) => Some(vec![Alphabet::A, Alphabet::A]),
          _ => None,
        },
      )],
    );
    assert_eq!(vec![Alphabet::A], system.apply().symbols);
  }

  #[test]
  fn when_a_contextual_rule_matches_only_the_target_symbol_is_replaced() {
    let system = LSystem::new(
      vec![Alphabet::B, Alphabet::A, Alphabet::C],
      vec![Production::SurroundingContext(
        &|pre, s, post| match (pre, s, post) {
          (Alphabet::B, Alphabet::A, Alphabet::C) => Some(vec![Alphabet::A, Alphabet::A]),
          _ => None,
        },
      )],
    );
    assert_eq!(
      vec![Alphabet::B, Alphabet::A, Alphabet::A, Alphabet::C],
      system.apply().symbols
    );
  }
}
