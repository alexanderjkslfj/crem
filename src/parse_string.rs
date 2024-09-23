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
    // converts "5 + (3*3) * -3" to [Term, '+', Term, '*', Term]
    let flat = parse_to_flat(value)?;
    // converts [Term, '+', Term, '*', Term] to Term
    Ok(fold_flat(flat))
}

fn fold_flat(input: Vec<Result<Term, char>>) -> Term {
    enum Operation {
        Add,
        Mul,
        Div,
    }

    let mut operation = Operation::Add;
    input
        .into_iter()
        .fold(Vec::new(), |mut acc, item| {
            match item {
                Ok(term) => match operation {
                    Operation::Add => {
                        acc.push(term);
                    }
                    Operation::Mul => {
                        let last_index = acc.len() - 1;
                        acc[last_index] *= term;
                    }
                    Operation::Div => {
                        let last_index = acc.len() - 1;
                        acc[last_index] /= term;
                    }
                },
                Err(char) => match char {
                    '+' => operation = Operation::Add,
                    '*' => operation = Operation::Mul,
                    '/' => operation = Operation::Div,
                    _ => panic!("Internal library error"),
                },
            }
            acc
        })
        .into_iter()
        .fold(Term::from(0u32), |acc, term| acc + term)
}

fn parse_to_flat(value: &str) -> Result<Vec<Result<Term, char>>, TryFromStrError> {
    let mut outputs = Vec::new();

    enum States {
        Start,    // Start of term; anything goes
        PostTerm, // After a term; expects an operation (or brackets)
        PostOp,   // After an operation; expects a term (or a minus)
        PostMinus(Box<States> /*previous state*/),
        PreComma(String /*buffer*/, bool /*positive number?*/), // Number before the comma
        PostComma(
            u32,    /*precomma value*/
            String, /*buffer*/
            bool,   /*positive number?*/
        ), // Number after the comma
        Brackets(
            usize,  /*bracket depth*/
            String, /*buffer*/
            bool,   /*positive?*/
        ), // A term inside brackets
    }

    let mut state = States::Start;
    for char in value.chars() {
        match state {
            States::Start => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state = States::PreComma(String::from(char), true);
                }
                '.' => {
                    state = States::PostComma(0, String::new(), true);
                }
                '+' | '*' | '/' => {
                    outputs.push(Err(char));
                    state = States::PostOp;
                }
                '-' => {
                    state = States::PostMinus(Box::new(States::Start));
                }
                '(' => {
                    state = States::Brackets(1, String::new(), true);
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostTerm => match char {
                '+' | '*' | '/' => {
                    outputs.push(Err(char));
                    state = States::PostOp;
                }
                '-' => {
                    state = States::PostMinus(Box::new(States::PostTerm));
                }
                '(' => {
                    outputs.push(Err('*'));
                    state = States::Brackets(1, String::new(), true);
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostOp => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state = States::PreComma(String::from(char), true);
                }
                '.' => {
                    state = States::PostComma(0, String::new(), true);
                }
                '-' => {
                    state = States::PostMinus(Box::new(States::PostOp));
                }
                '(' => {
                    state = States::Brackets(1, String::new(), true);
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PreComma(mut buffer, positive) => match char {
                '.' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    state = States::PostComma(parsed_buffer, String::new(), positive);
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    buffer.push(char);
                    state = States::PreComma(buffer, positive);
                }
                '(' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term = Term::from(parsed_buffer);
                    outputs.push(Ok(if positive { term } else { -term }));
                    outputs.push(Err('*'));
                    state = States::Brackets(1, String::new(), true);
                }
                '+' | '-' | '*' | '/' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term = Term::from(parsed_buffer);
                    outputs.push(Ok(if positive { term } else { -term }));
                    if char == '-' {
                        state = States::PostMinus(Box::new(States::PostTerm));
                    } else {
                        outputs.push(Err(char));
                        state = States::PostOp;
                    }
                }
                any if any.is_whitespace() => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term = Term::from(parsed_buffer);
                    outputs.push(Ok(if positive { term } else { -term }));
                    state = States::PostTerm;
                }
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostComma(pre, mut buffer, positive) => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    buffer.push(char);
                    state = States::PostComma(pre, buffer, positive);
                }
                '(' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term =
                        Term::from(pre) + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32));
                    outputs.push(Ok(if positive { term } else { -term }));
                    outputs.push(Err('*'));
                    state = States::Brackets(1, String::new(), true);
                }
                '+' | '-' | '*' | '/' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term =
                        Term::from(pre) + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32));
                    outputs.push(Ok(if positive { term } else { -term }));
                    if char == '-' {
                        state = States::PostMinus(Box::new(States::PostTerm));
                    } else {
                        outputs.push(Err(char));
                        state = States::PostOp;
                    }
                }
                any if any.is_whitespace() => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    let term =
                        Term::from(pre) + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32));
                    outputs.push(Ok(if positive { term } else { -term }));
                    state = States::PostTerm;
                }
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::Brackets(depth, mut buffer, positive) => match char {
                '(' => {
                    buffer.push('(');
                    state = States::Brackets(depth + 1, buffer, positive);
                }
                ')' => {
                    if depth == 1 {
                        let term = parse_string(buffer.as_str())?;
                        outputs.push(Ok(if positive { term } else { -term }));
                        state = States::PostTerm;
                    } else {
                        buffer.push(')');
                        state = States::Brackets(depth - 1, buffer, positive);
                    }
                }
                any => {
                    buffer.push(any);
                    state = States::Brackets(depth, buffer, positive);
                }
            },
            States::PostMinus(ref prev_state) => match char {
                '-' => {
                    match prev_state.as_ref() {
                        States::Start => {
                            state = States::Start;
                        }
                        States::PostOp => {
                            state = States::PostOp;
                        }
                        States::PostTerm => {
                            outputs.push(Err('+'));
                            state = States::PostOp;
                        }
                        _ => panic!("Internal library error."),
                    };
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if matches!(prev_state.as_ref(), States::PostTerm) {
                        outputs.push(Err('+'));
                    }
                    state = States::PreComma(String::from(char), false);
                }
                '.' => {
                    if matches!(prev_state.as_ref(), States::PostTerm) {
                        outputs.push(Err('+'));
                    }
                    state = States::PostComma(0, String::new(), false);
                }
                '(' => {
                    if matches!(prev_state.as_ref(), States::PostTerm) {
                        outputs.push(Err('+'));
                    }
                    state = States::Brackets(1, String::new(), false);
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
        }
    }

    match state {
        States::Brackets(_, _, _) | States::PostMinus(_) | States::PostOp | States::Start => {
            return Err(TryFromStrError::UnexpectedEof)
        }
        States::PostComma(pre, buffer, positive) => {
            let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                panic!("Internal library error.");
            };
            let term = Term::from(pre) + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32));
            outputs.push(Ok(if positive { term } else { -term }));
        }
        States::PreComma(buffer, positive) => {
            let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                panic!("Internal library error.");
            };
            let term = Term::from(parsed_buffer);
            outputs.push(Ok(if positive { term } else { -term }));
        }
        States::PostTerm => (),
    };

    Ok(outputs)
}
