//! Turing Machine
//!
//!  M = (Q, Σ, q0, δ)
//!
//!  Q: state set
//!
//!  Σ: symbol set
//!
//!  q0: initial state (q0 ∈ Q)
//!
//!  δ: transition function set (δ: Q × Σ → Q × Σ × {L, R, S})

type Tape<Σ> = Vec<Option<Σ>>;

#[derive(Eq, PartialEq, Clone)]
pub enum HeadMovementDirection {
    Left,
    Right,
    Stay,
}

#[allow(non_camel_case_types)]
type δ<Q, Σ> = Vec<δEl<Q, Σ>>;

#[allow(non_camel_case_types)]
pub struct δEl<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display> {
    test: δFnTest<Q, Σ>,
    action: δFnAction<Q, Σ>,
}
impl<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display> δEl<Q, Σ> {
    pub fn new(test: δFnTest<Q, Σ>, action: δFnAction<Q, Σ>) -> δEl<Q, Σ> {
        δEl { test, action }
    }
}

#[allow(non_camel_case_types)]
pub struct δFnTest
<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display> {
    q: Q,
    read: Option<Σ>,
}
impl<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display>
    δFnTest<Q, Σ>
{
    pub fn new(q: Q, read: Option<Σ>) -> δFnTest<Q, Σ> {
        δFnTest { q, read }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct δFnAction<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display>
{
    write_to: Option<Σ>,
    direction: HeadMovementDirection,
    q: Q,
    acceptance: bool,
}
impl<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display>
    δFnAction<Q, Σ>
{
    pub fn new(
        write_to: Option<Σ>,
        direction: HeadMovementDirection,
        q: Q,
        acceptance: bool,
    ) -> δFnAction<Q, Σ> {
        δFnAction {
            write_to,
            direction,
            q,
            acceptance,
        }
    }
}

pub struct TuringMachine
<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display> {
    tape: Tape<Σ>,
    head_position: usize,
    q: Q,
    δ: δ<Q, Σ>,
}
impl<Q: Eq + PartialEq + Clone, Σ: Eq + PartialEq + Clone + std::fmt::Display> TuringMachine<Q, Σ> {
    pub fn new(
        δ: Vec<δEl<Q, Σ>>,
        q0: Q,
        tape: Tape<Σ>,
        head_position: usize,
    ) -> TuringMachine<Q, Σ> {
        TuringMachine {
            tape,
            head_position,
            q: q0,
            δ,
        }
    }

    pub fn set(&mut self, q0: Q, tape: Tape<Σ>, head_position: usize) {
        self.q = q0;
        self.tape = tape;
        self.head_position = head_position;
    }

    pub fn start(&mut self) {
        loop {
            let read = self.tape[self.head_position].clone();

            let action = self
                .δ
                .iter()
                .find(|x| x.test.q == self.q && x.test.read == read)
                .unwrap()
                .action
                .clone();

            // action
            self.tape[self.head_position] = action.write_to;
            match action.direction {
                HeadMovementDirection::Left => {
                    if self.head_position == 0 {
                        self.tape.insert(0, None);
                    } else {
                        self.head_position -= 1;
                    }
                }
                HeadMovementDirection::Right => {
                    if self.head_position == self.tape.len() - 1 {
                        self.tape.push(None);
                    }
                    self.head_position += 1;
                }
                HeadMovementDirection::Stay => {}
            }
            if action.acceptance {
                break;
            }
            self.q = action.q;
        }
    }

    pub fn print(&self) {
        self.tape.iter().for_each(|x| match x {
            Some(x) => print!("{}", x),
            None => print!(" "),
        });
        println!()
    }
}
