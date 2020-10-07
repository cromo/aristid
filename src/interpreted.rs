use crate::compiled;
use std::fmt;

/// Represents a symbol with a given label and optional set of parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
  pub label: String,
  // An ordered collection of values (doubles?) needs to be added. This allows
  // each symbol to carry its parameters with it, enabling parametricity.
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.label)
  }
}

pub struct Production {
  pub pattern: (Option<SymbolPattern>, SymbolPattern, Option<SymbolPattern>),
  // This needs an optional guard, which will be a boolean expression (which is
  // distinct from a SymbolExpression). This is an extension that's only useful
  // with parametricity.
  pub replacement_expression: Vec<SymbolExpression>,
}

/// Represents a symbol and an optional set of patterns that can be literals or
/// placeholders.
pub struct SymbolPattern {
  pub label: String,
  // An ordered collection of values (for literal matching)/labels (for pattern
  // binding) needs to be added to support parametricity.
}

pub struct SymbolExpression {
  pub label: String,
  // Need to add an ordered collection of arithmetic expressions to support
  // parametricity.
}

// This needs to be extended to accept a mutable environment map from labels to
// values.
fn pattern_matches(pattern: &SymbolPattern, symbol: &Symbol) -> bool {
  pattern.label == symbol.label
  // This needs to be extended check the symbol parameters against the pattern's
  // parameters for parametricity.
  // If the pattern is a literal, then it should compare it against the
  // corresponding symbol value and continue if they're equal.
  // If the pattern is a label, then it should check the environment for that
  // label. If present in the environment, it should compare the environment
  // value to the corresponding symbol value and continue if they're equal. If
  // not present in the environment, then it should add the label and
  // corresponding symbol value to the environment and continue.
}

// This needs to be extended to accept an environment map from labels to values.
fn eval(expression: &SymbolExpression) -> Symbol {
  Symbol {
    label: expression.label.clone(),
  }
}

pub fn new(axiom: Vec<Symbol>, productions: Vec<Production>) -> compiled::LSystem<Symbol> {
  compiled::LSystem::new(
    axiom,
    productions
      .into_iter()
      .map(|p| match (p.pattern, p.replacement_expression) {
        ((None, target, None), replacement) => {
          compiled::Production::ContextFree(Box::new(move |s| {
            // After the environment is build up by pattern matching, it should
            // use it to run the guard.
            if pattern_matches(&target, s) {
              Some(replacement.iter().map(eval).collect())
            } else {
              None
            }
          }))
        }
        ((Some(pre), target, None), replacement) => {
          compiled::Production::PriorContext(Box::new(move |b, s| {
            if pattern_matches(&pre, b) && pattern_matches(&target, s) {
              Some(replacement.iter().map(eval).collect())
            } else {
              None
            }
          }))
        }
        ((None, target, Some(post)), replacement) => {
          compiled::Production::FollowingContext(Box::new(move |s, a| {
            if pattern_matches(&target, s) && pattern_matches(&post, a) {
              Some(replacement.iter().map(eval).collect())
            } else {
              None
            }
          }))
        }
        ((Some(pre), target, Some(post)), replacement) => {
          compiled::Production::SurroundingContext(Box::new(move |b, s, a| {
            if pattern_matches(&pre, b) && pattern_matches(&target, s) && pattern_matches(&post, a)
            {
              Some(replacement.iter().map(eval).collect())
            } else {
              None
            }
          }))
        }
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_matching_rules_produces_same_symbol() {
    let system = new(
      vec![Symbol {
        label: String::from("A"),
      }],
      vec![Production {
        pattern: (
          None,
          SymbolPattern {
            label: String::from("B"),
          },
          None,
        ),
        replacement_expression: vec![SymbolExpression {
          label: String::from("C"),
        }],
      }],
    );
    assert_eq!(
      vec![Symbol {
        label: String::from("A")
      }],
      system.apply().symbols
    );
  }

  #[test]
  fn matching_rule_is_applied() {
    let system = new(
      vec![Symbol {
        label: String::from("A"),
      }],
      vec![Production {
        pattern: (
          None,
          SymbolPattern {
            label: String::from("A"),
          },
          None,
        ),
        replacement_expression: vec![SymbolExpression {
          label: String::from("B"),
        }],
      }],
    );
    assert_eq!(
      vec![Symbol {
        label: String::from("B")
      }],
      system.apply().symbols
    );
  }

  #[test]
  fn when_a_contextual_rule_matches_only_the_target_symbol_is_replaced() {
    let system = new(
      vec![
        Symbol {
          label: String::from("B"),
        },
        Symbol {
          label: String::from("A"),
        },
        Symbol {
          label: String::from("C"),
        },
      ],
      vec![Production {
        pattern: (
          Some(SymbolPattern {
            label: String::from("B"),
          }),
          SymbolPattern {
            label: String::from("A"),
          },
          Some(SymbolPattern {
            label: String::from("C"),
          }),
        ),
        replacement_expression: vec![
          SymbolExpression {
            label: String::from("A"),
          },
          SymbolExpression {
            label: String::from("A"),
          },
        ],
      }],
    );
    assert_eq!(
      vec![
        Symbol {
          label: String::from("B")
        },
        Symbol {
          label: String::from("A")
        },
        Symbol {
          label: String::from("A")
        },
        Symbol {
          label: String::from("C")
        },
      ],
      system.apply().symbols
    );
  }
}

/*
Once we have a set of Symbols and Productions, we can do some basic checking
to make sure that every Symbol with the same label has the same arity.
*/
