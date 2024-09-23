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
    // converts "5 + (3*3) * -3" to [Term, '+', Term, '*', '-', Term]
    let mut flat = parse_to_flat(value)?;
    // converts [Term, '+', Term, '*', '-', Term] to [Term, '+', Term, '*', Term]
    remove_minuses(&mut flat);
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

fn remove_minuses(input: &mut Vec<Result<Term, char>>) {
    let mut i = 0;
    while i < input.len() {
        let Err(char) = input[i] else {
            i += 1;
            continue;
        };
        if char == '-' {
            match &input[i + 1] {
                Err(_) => {
                    let _ = input.remove(i + 1);
                }
                Ok(_) => {
                    let term = input.remove(i + 1);
                    input.insert(i + 1, Ok(-term.unwrap()));
                }
            }
            match &input[i - 1] {
                Err(_) => {
                    let _ = input.remove(i);
                }
                Ok(_) => {
                    input[i] = Err('+');
                }
            };
        } else {
            i += 1;
        }
    }
}

fn parse_to_flat(value: &str) -> Result<Vec<Result<Term, char>>, TryFromStrError> {
    let mut outputs = Vec::new();

    enum States {
        Start,                                                // Start of term; anything goes
        PostTerm,                    // After a term; expects an operation (or brackets)
        PostOp,                      // After an operation; expects a term (or a minus)
        PreComma(String /*buffer*/), // Number before the comma
        PostComma(u32 /*precomma value*/, String /*buffer*/), // Number after the comma
        Brackets(usize /*bracket depth*/, String /*buffer*/), // A term inside brackets
    }

    let mut state = States::Start;
    for char in value.chars() {
        match state {
            States::Start => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state = States::PreComma(String::from(char));
                }
                '.' => {
                    state = States::PostComma(0, String::new());
                }
                '+' | '-' | '*' | '/' => {
                    outputs.push(Err(char));
                    state = States::PostOp;
                }
                '(' => {
                    state = States::Brackets(1, String::new());
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostTerm => match char {
                '+' | '-' | '*' | '/' => {
                    outputs.push(Err(char));
                    state = States::PostOp;
                }
                '(' => {
                    outputs.push(Err('*'));
                    state = States::Brackets(1, String::new());
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostOp => match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    state = States::PreComma(String::from(char));
                }
                '.' => {
                    state = States::PostComma(0, String::new());
                }
                '-' => {
                    outputs.push(Err(char));
                }
                '(' => {
                    state = States::Brackets(1, String::new());
                }
                any if any.is_whitespace() => (),
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PreComma(mut buffer) => match char {
                '.' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    state = States::PostComma(parsed_buffer, String::new());
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    buffer.push(char);
                    state = States::PreComma(buffer);
                }
                '(' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    outputs.push(Ok(Term::from(parsed_buffer)));
                    outputs.push(Err('*'));
                    state = States::Brackets(1, String::new());
                }
                '+' | '-' | '*' | '/' => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    outputs.push(Ok(Term::from(parsed_buffer)));
                    outputs.push(Err(char));
                    state = States::PostOp;
                }
                any if any.is_whitespace() => {
                    let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                        panic!("Internal library error.");
                    };
                    outputs.push(Ok(Term::from(parsed_buffer)));
                    state = States::PostTerm;
                }
                any => return Err(TryFromStrError::UnexpectedCharacter(any)),
            },
            States::PostComma(pre, mut buffer) => {
                match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        buffer.push(char);
                        state = States::PostComma(pre, buffer);
                    }
                    '(' => {
                        let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                            panic!("Internal library error.");
                        };
                        outputs.push(Ok(Term::from(pre)
                            + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32))));
                        outputs.push(Err('*'));
                        state = States::Brackets(1, String::new());
                    }
                    '+' | '-' | '*' | '/' => {
                        let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                            panic!("Internal library error.");
                        };
                        outputs.push(Ok(Term::from(pre)
                            + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32))));
                        outputs.push(Err(char));
                        state = States::PostOp;
                    }
                    any if any.is_whitespace() => {
                        let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                            panic!("Internal library error.");
                        };
                        outputs.push(Ok(Term::from(pre)
                            + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32))));
                        state = States::PostTerm;
                    }
                    any => return Err(TryFromStrError::UnexpectedCharacter(any)),
                }
            }
            States::Brackets(depth, mut buffer) => match char {
                '(' => {
                    buffer.push('(');
                    state = States::Brackets(depth + 1, buffer);
                }
                ')' => {
                    if depth == 1 {
                        outputs.push(Ok(parse_string(buffer.as_str())?));
                        state = States::PostTerm;
                    } else {
                        buffer.push(')');
                        state = States::Brackets(depth - 1, buffer);
                    }
                }
                any => {
                    buffer.push(any);
                    state = States::Brackets(depth, buffer);
                }
            },
        }
    }

    match state {
        States::Brackets(_, _) | States::PostOp | States::Start => {
            return Err(TryFromStrError::UnexpectedEof)
        }
        States::PostComma(pre, buffer) => {
            let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                panic!("Internal library error.");
            };
            outputs.push(Ok(
                Term::from(pre) + Term::div(parsed_buffer, 10u32.pow(buffer.len() as u32))
            ));
        }
        States::PreComma(buffer) => {
            let Ok(parsed_buffer) = buffer.parse::<u32>() else {
                panic!("Internal library error.");
            };
            outputs.push(Ok(Term::from(parsed_buffer)));
        }
        States::PostTerm => (),
    };

    Ok(outputs)
}
