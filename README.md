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
    Blank,
}
impl std::fmt::Display for Σ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Σ::Zero => write!(f, "0"),
            Σ::One => write!(f, "1"),
            Σ::Blank => write!(f, " "),
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
        vec![
            δEl::new(
                δFnTest::new(Q::Do, Σ::One),
                δFnAction::new(Σ::Zero, HeadMovementDirection::Left, Q::Do, false),
            ),
            δEl::new(
                δFnTest::new(Q::Do, Σ::Zero),
                δFnAction::new(Σ::One, HeadMovementDirection::Left, Q::Stop, true),
            ),
            δEl::new(
                δFnTest::new(Q::Do, Σ::Blank),
                δFnAction::new(Σ::Zero, HeadMovementDirection::Left, Q::Stop, true),
            ),
        ],
        Q::Do,
        vec![Σ::Zero, Σ::One, Σ::One, Σ::One],
        3,
    );
    m.start();
    m.print(); // 1000

    /*
     *
     * calc 1000000000001111 + 1
     *
     */
    m.set(
        Q::Do,
        vec![
            Σ::One,
            Σ::Zero,
            Σ::Zero,
            Σ::Zero,
            //
            Σ::Zero,
            Σ::Zero,
            Σ::Zero,
            Σ::Zero,
            //
            Σ::Zero,
            Σ::Zero,
            Σ::Zero,
            Σ::Zero,
            //
            Σ::One,
            Σ::One,
            Σ::One,
            Σ::One,
        ],
        15,
    );
    m.start();
    m.print(); // 1000000000010000
}
```

## Examples

```sh
$ cargo run --example binary_add_one
```
