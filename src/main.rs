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
        "Algae:\nAxiom: {:?}\nP1: {:?}\nP2: {:?}\nP3: {:?}",
        algae.symbols,
        algae.apply().symbols,
        algae.apply().apply().symbols,
        algae.apply().apply().apply().symbols
    );

    let fractal_tree = aristid::LSystem::new(
        vec![BinaryTree::Zero],
        vec![
            &|s| match s {
                BinaryTree::One => Some(vec![BinaryTree::One, BinaryTree::One]),
                _ => None,
            },
            &|s| match s {
                BinaryTree::Zero => Some(vec![
                    BinaryTree::One,
                    BinaryTree::Push,
                    BinaryTree::Zero,
                    BinaryTree::Pop,
                    BinaryTree::Zero,
                ]),
                _ => None,
            },
        ],
    );
    println!(
        "Binary tree:\nAxiom: {:?}\nP1: {:?}\nP2: {:?}\nP3: {:?}",
        fractal_tree.symbols,
        fractal_tree.apply().symbols,
        fractal_tree.apply().apply().symbols,
        fractal_tree.apply().apply().apply().symbols
    );

    let koch_curve = aristid::LSystem::new(
        vec![KochCurve::F],
        vec![&|s| match s {
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
        }],
    );
    println!(
        "Koch curve:\nAxiom: {:?}\nP1: {:?}\nP2: {:?}\nP3: {:?}",
        koch_curve.symbols,
        koch_curve.apply().symbols,
        koch_curve.apply().apply().symbols,
        koch_curve.apply().apply().apply().symbols
    );
}

#[derive(Clone, Copy, Debug)]
enum Algae {
    A,
    B,
}

#[derive(Clone, Copy, Debug)]
enum BinaryTree {
    Zero,
    One,
    Push,
    Pop,
}

#[derive(Clone, Copy, Debug)]
enum KochCurve {
    F,
    Plus,
    Minus,
}
