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
        Q::Do,
        vec![Some(Σ::Zero), Some(Σ::One), Some(Σ::One), Some(Σ::One)],
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
            Some(Σ::One),
            Some(Σ::Zero),
            Some(Σ::Zero),
            Some(Σ::Zero),
            //
            Some(Σ::Zero),
            Some(Σ::Zero),
            Some(Σ::Zero),
            Some(Σ::Zero),
            //
            Some(Σ::Zero),
            Some(Σ::Zero),
            Some(Σ::Zero),
            Some(Σ::Zero),
            //
            Some(Σ::One),
            Some(Σ::One),
            Some(Σ::One),
            Some(Σ::One),
        ],
        15,
    );
    m.start();
    m.print(); // 1000000000010000
}
