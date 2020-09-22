use std::fmt;

fn main() {
    let algae = aristid::LSystem::new(
        vec![Algae::A],
        vec![
            aristid::Production::ContextFree(&|s| match s {
                Algae::A => Some(vec![Algae::A, Algae::B]),
                _ => None,
            }),
            aristid::Production::ContextFree(&|s| match s {
                Algae::B => Some(vec![Algae::A]),
                _ => None,
            }),
        ],
    );
    println!(
        "Algae:\n  Axiom: {}\n     P1: {}\n     P2: {}\n     P3: {}",
        to_string(&algae),
        to_string(&algae.apply()),
        to_string(&algae.apply().apply()),
        to_string(&algae.apply().apply().apply())
    );

    let fractal_tree = aristid::LSystem::new(
        vec![BinaryTree::Zero],
        vec![
            aristid::Production::ContextFree(&|s| match s {
                BinaryTree::One => Some(vec![BinaryTree::One, BinaryTree::One]),
                _ => None,
            }),
            aristid::Production::ContextFree(&|s| match s {
                BinaryTree::Zero => Some(vec![
                    BinaryTree::One,
                    BinaryTree::Push,
                    BinaryTree::Zero,
                    BinaryTree::Pop,
                    BinaryTree::Zero,
                ]),
                _ => None,
            }),
        ],
    );
    println!(
        "Binary tree:\n  Axiom: {}\n     P1: {}\n     P2: {}\n     P3: {}",
        to_string(&fractal_tree),
        to_string(&fractal_tree.apply()),
        to_string(&fractal_tree.apply().apply()),
        to_string(&fractal_tree.apply().apply().apply())
    );

    let koch_curve = aristid::LSystem::new(
        vec![KochCurve::F],
        vec![aristid::Production::ContextFree(&|s| match s {
            KochCurve::F => Some(vec![
                KochCurve::F,
                KochCurve::Plus,
                KochCurve::F,
                KochCurve::Minus,
                KochCurve::F,
                KochCurve::Minus,
                KochCurve::F,
                KochCurve::Plus,
                KochCurve::F,
            ]),
            _ => None,
        })],
    );
    println!(
        "Koch curve:\n  Axiom: {}\n     P1: {}\n     P2: {}\n     P3: {}",
        to_string(&koch_curve),
        to_string(&koch_curve.apply()),
        to_string(&koch_curve.apply().apply()),
        to_string(&koch_curve.apply().apply().apply())
    );

    let parametric = aristid::LSystem::new(
        vec![Parametric::A(0, 2)],
        vec![aristid::Production::ContextFree(&|s| match s {
            Parametric::A(x, y) if *x == 0u8 => {
                Some(vec![Parametric::A(1, y + 1), Parametric::B(2, 3)])
            }
            _ => None,
        })],
    );
    println!(
        "Parametric:\n  Axiom: {}\n     P1: {}",
        to_string(&parametric),
        to_string(&parametric.apply()),
    );
}

fn to_string<T: fmt::Display>(system: &aristid::LSystem<T>) -> String {
    system
        .symbols
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}

impl fmt::Display for Algae {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algae::A => write!(f, "A"),
            Algae::B => write!(f, "B"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BinaryTree {
    Zero,
    One,
    Push,
    Pop,
}

impl fmt::Display for BinaryTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryTree::Zero => write!(f, "0"),
            BinaryTree::One => write!(f, "1"),
            BinaryTree::Push => write!(f, "["),
            BinaryTree::Pop => write!(f, "]"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum KochCurve {
    F,
    Plus,
    Minus,
}

impl fmt::Display for KochCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KochCurve::F => write!(f, "F"),
            KochCurve::Plus => write!(f, "+"),
            KochCurve::Minus => write!(f, "-"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Parametric {
    A(u8, u8),
    B(u8, u8),
}

impl fmt::Display for Parametric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parametric::A(x, y) => write!(f, "A({},{})", x, y),
            Parametric::B(x, y) => write!(f, "B({},{})", x, y),
        }
    }
}
