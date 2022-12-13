#[derive(Clone, Eq, PartialEq)]
enum Sigma {
    Zero,
    One,
    Blank,
}
impl std::fmt::Display for Sigma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sigma::Zero => write!(f, "0"),
            Sigma::One => write!(f, "1"),
            Sigma::Blank => write!(f, " "),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Stay,
}

struct DeltaFunction<Q: Eq + PartialEq + Clone> {
    test: DeltaFunctionTest<Q>,
    action: DeltaFunctionAction<Q>,
}
struct DeltaFunctionTest<Q: Eq + PartialEq + Clone> {
    q: Q,
    read: Sigma,
}

#[derive(Clone)]
struct DeltaFunctionAction<Q: Eq + PartialEq + Clone> {
    write: Sigma,
    direction: Direction,
    q: Q,
    acceptance: bool,
}

struct TuringMachine<Q: Eq + PartialEq + Clone> {
    tape: Vec<Sigma>,
    head_position: usize,
    q: Q,
    delta: Vec<DeltaFunction<Q>>,
}

impl<Q: Eq + PartialEq + Clone> TuringMachine<Q> {
    fn new(
        delta: Vec<DeltaFunction<Q>>,
        q: Q,
        tape: Vec<Sigma>,
        head_position: usize,
    ) -> TuringMachine<Q> {
        TuringMachine {
            tape,
            head_position,
            q,
            delta,
        }
    }

    fn set(&mut self, q: Q, tape: Vec<Sigma>, head_position: usize) {
        self.q = q;
        self.tape = tape;
        self.head_position = head_position;
    }

    fn start(&mut self) {
        loop {
            let read = self.tape[self.head_position].clone();

            let action = self
                .delta
                .iter()
                .find(|x| x.test.q == self.q && x.test.read == read)
                .unwrap()
                .action
                .clone();

            // action
            self.tape[self.head_position] = action.write;
            match action.direction {
                Direction::Left => {
                    if self.head_position == 0 {
                        self.tape.insert(0, Sigma::Blank);
                    } else {
                        self.head_position -= 1;
                    }
                }
                Direction::Right => {
                    if self.head_position == self.tape.len() - 1 {
                        self.tape.push(Sigma::Blank);
                    }
                    self.head_position += 1;
                }
                Direction::Stay => {}
            }
            if action.acceptance {
                break;
            }
            self.q = action.q;
        }
    }

    fn print(&self) {
        self.tape.iter().for_each(|x| print!("{}", x));
        println!()
    }
}

fn main() {
    #[derive(Clone, Eq, PartialEq)]
    enum State {
        Do,
        Stop,
    }
    let delta = vec![
        DeltaFunction {
            test: DeltaFunctionTest {
                q: State::Do,
                read: Sigma::One,
            },
            action: DeltaFunctionAction {
                write: Sigma::Zero,
                direction: Direction::Left,
                q: State::Do,
                acceptance: false,
            },
        },
        DeltaFunction {
            test: DeltaFunctionTest {
                q: State::Do,
                read: Sigma::Zero,
            },
            action: DeltaFunctionAction {
                write: Sigma::One,
                direction: Direction::Left,
                q: State::Stop,
                acceptance: true,
            },
        },
        DeltaFunction {
            test: DeltaFunctionTest {
                q: State::Do,
                read: Sigma::Blank,
            },
            action: DeltaFunctionAction {
                write: Sigma::Zero,
                direction: Direction::Left,
                q: State::Stop,
                acceptance: true,
            },
        },
    ];

    let mut tm_add_one = TuringMachine::new(
        delta,
        State::Do,
        vec![Sigma::Zero, Sigma::One, Sigma::One, Sigma::One],
        3,
    );
    tm_add_one.start();
    tm_add_one.print();

    tm_add_one.set(
        State::Do,
        vec![
            Sigma::One,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::Zero,
            Sigma::One,
            Sigma::One,
            Sigma::One,
            Sigma::One,
        ],
        15,
    );
    tm_add_one.start();
    tm_add_one.print();
}
