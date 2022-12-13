# Turing Machine by Rust

## Formal Definition

```
M = (Q, Σ, q0, δ)
    Q: state set
    Σ: symbol set
    q0: initial state (q0 ∈ Q)
    δ: transition function set (δ: Q × Σ → Q × Σ × {L, R, S})
```

## Usage

```rs
use rusty_turing::*;

#[derive(Clone, Eq, PartialEq)]
enum Σ {
    Zero,
    One,
}
impl std::fmt::Display for Σ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Σ::Zero => write!(f, "0"),
            Σ::One => write!(f, "1"),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Q {
    Do,
    Stop,
}

fn main() {
    /*
     *
     * calc 0111 + 1
     *
     */
    let mut m = TuringMachine::new(
        // δ
        vec![
            δEl::new(
                DeltaFunctionTest::new(Q::Do, Some(Σ::One)),
                DeltaFunctionAction::new(Some(Σ::Zero), HeadMovementDirection::Left, Q::Do, false),
            ),
            δEl::new(
                DeltaFunctionTest::new(Q::Do, Some(Σ::Zero)),
                DeltaFunctionAction::new(Some(Σ::One), HeadMovementDirection::Left, Q::Stop, true),
            ),
            δEl::new(
                DeltaFunctionTest::new(Q::Do, None),
                DeltaFunctionAction::new(Some(Σ::Zero), HeadMovementDirection::Left, Q::Stop, true),
            ),
        ],

        // q0
        Q::Do,

        // tape
        vec![Some(Σ::Zero), Some(Σ::One), Some(Σ::One), Some(Σ::One)],

        // initial head position
        3,
    );

    m.start();

    m.print(); // 1000
}

```

## Examples

```sh
$ cargo run --example binary_add_one
```
