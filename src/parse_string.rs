use crate::Term;

/// Error when creating a term from a string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TryFromStrError {
    /// An illegal character was encountered
    UnexpectedCharacter(char),
    /// The EOF was reached while some operations or brackets were still open
    UnexpectedEof,
}

pub fn parse_string(value: &str) -> Result<Term, TryFromStrError> {
    let mut outputs = Vec::new();

    enum Operation {
        Add,
        Mul,
        Div,
    }

    impl TryFrom<char> for Operation {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '+' => Ok(Operation::Add),
                '*' => Ok(Operation::Mul),
                '/' => Ok(Operation::Div),
                _ => Err(()),
            }
        }
    }

    enum Value {
        None,
        PreComma(String),
        PostComma(u32, String),
        Brackets(usize, String),
    }

    enum State {
        Start,
        PostTerm,
        Term(
            Operation, /*operation*/
            bool,      /*negated?*/
            Value,     /*value state*/
        ),
    }

    let mut add_to_output = |operation: Operation, negated: bool, term: Term| {
        let t = if negated { -term } else { term };
        match operation {
            Operation::Add => {
                outputs.push(t);
            }
            Operation::Mul => {
                let last_index = outputs.len() - 1;
                outputs[last_index] *= t;
            }
            Operation::Div => {
                let last_index = outputs.len() - 1;
                outputs[last_index] /= t;
            }
        }
    };

    let mut state = State::Start;
    for char in value.chars() {
        state = match state {
            State::Start => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    State::Term(Operation::Add, false, Value::PreComma(char.into()))
                }
                '.' => State::Term(Operation::Add, false, Value::PostComma(0, String::new())),
                '+' | '*' | '/' => {
                    State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                }
                '-' => State::Term(Operation::Add, true, Value::None),
                '(' => State::Term(Operation::Add, false, Value::Brackets(1, String::new())),
                any if any.is_whitespace() => state,
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            State::PostTerm => match char {
                '+' | '*' | '/' => {
                    State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                }
                '-' => State::Term(Operation::Add, true, Value::None),
                '(' => State::Term(Operation::Mul, false, Value::Brackets(1, String::new())),
                any if any.is_whitespace() => state,
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            State::Term(op, neg, val) => match val {
                Value::None => match char {
                    '-' => State::Term(op, !neg, val),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        State::Term(op, neg, Value::PreComma(char.into()))
                    }
                    '.' => State::Term(op, neg, Value::PostComma(0, char.into())),
                    '(' => State::Term(op, neg, Value::Brackets(1, String::new())),
                    any if any.is_whitespace() => State::Term(op, neg, Value::None),
                    any => return Err(TryFromStrError::UnexpectedCharacter(any)),
                },
                Value::Brackets(depth, mut buffer) => match char {
                    '(' => {
                        buffer.push('(');
                        State::Term(op, neg, Value::Brackets(depth + 1, buffer))
                    }
                    ')' => {
                        if depth == 1 {
                            add_to_output(op, neg, parse_string(&buffer)?);
                            State::PostTerm
                        } else {
                            buffer.push(')');
                            State::Term(op, neg, Value::Brackets(depth - 1, buffer))
                        }
                    }
                    any => {
                        buffer.push(any);
                        State::Term(op, neg, Value::Brackets(depth, buffer))
                    }
                },
                Value::PreComma(mut buffer) => match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        buffer.push(char);
                        State::Term(op, neg, Value::PreComma(buffer))
                    }
                    '.' => State::Term(
                        op,
                        neg,
                        Value::PostComma(buffer.parse::<u32>().unwrap(), String::new()),
                    ),
                    '+' | '*' | '/' => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        add_to_output(op, neg, term);
                        State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                    }
                    '-' => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        add_to_output(op, neg, term);
                        State::Term(Operation::Add, true, Value::None)
                    }
                    '(' => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        add_to_output(op, neg, term);
                        State::Term(Operation::Mul, false, Value::Brackets(1, String::new()))
                    }
                    any if any.is_whitespace() => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        add_to_output(op, neg, term);
                        State::PostTerm
                    }
                    any => return Err(TryFromStrError::UnexpectedCharacter(any)),
                },
                Value::PostComma(pre, mut buffer) => match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        buffer.push(char);
                        State::Term(op, neg, Value::PostComma(pre, buffer))
                    }
                    '+' | '*' | '/' => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        add_to_output(op, neg, term);
                        State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                    }
                    '-' => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        add_to_output(op, neg, term);
                        State::Term(Operation::Add, true, Value::None)
                    }
                    '(' => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        add_to_output(op, neg, term);
                        State::Term(Operation::Mul, false, Value::Brackets(1, String::new()))
                    }
                    any if any.is_whitespace() => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        add_to_output(op, neg, term);
                        State::PostTerm
                    }
                    any => return Err(TryFromStrError::UnexpectedCharacter(any)),
                },
            },
        }
    }

    // cleanup leftover state
    match state {
        State::Start => return Err(TryFromStrError::UnexpectedEof),
        State::Term(op, neg, val) => match val {
            Value::None | Value::Brackets(_, _) => return Err(TryFromStrError::UnexpectedEof),
            Value::PreComma(buffer) => {
                let term = Term::from(buffer.parse::<u32>().unwrap());
                add_to_output(op, neg, term);
            }
            Value::PostComma(pre, buffer) => {
                let term = Term::from(pre)
                    + Term::div(
                        buffer.parse::<u32>().unwrap(),
                        10u32.pow(buffer.len() as u32),
                    );
                add_to_output(op, neg, term);
            }
        },
        State::PostTerm => (),
    }

    Ok(outputs
        .into_iter()
        .fold(Term::from(0u32), |acc, term| acc + term))
}
