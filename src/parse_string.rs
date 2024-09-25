use std::mem::take;

use crate::Term;

/// Error when creating a term from a string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TryFromStrError {
    /// An illegal character was encountered
    UnexpectedCharacter(char),
    /// The EOF was reached while some operations or brackets were still open
    UnexpectedEof,
}

/// Parses a formular. Used in `impl TryFrom<&str> for Term`.
///
/// Uses a state machine internally.
///
/// Expected behavior:
/// ```rust
/// # use crem::*;
/// assert_eq!(Term::try_from("2 + 3")?, Term::from(2) + Term::from(3));
/// assert_eq!(Term::try_from("2 + 3")?, Term::from(5));
/// # Ok::<(), TryFromStrError>(())
/// ```
pub fn parse_string(value: &str) -> Result<Term<u32>, TryFromStrError> {
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

    /// The current state of a value (an operation will be applied to).
    /// A value is either a term contained within brackets or a number.
    enum Value {
        /// The value has not started being read yet.
        None,
        /// A number has started being read.
        /// The digits read so far are stored in the buffer.
        /// A comma has not been encountered.
        PreComma(String /* buffer */),
        /// A number has started being read, after a comma was encountered.
        /// The post-comma digits read so far are stored in the buffer.
        /// The number before the comma is also stored.
        PostComma(u32 /* pre-comma number */, String /* buffer */),
        /// The value is a term within brackets.
        /// Anything within the outer-most brackets is stored in the buffer.
        /// The depth counts the bracket depth. It starts at 1.
        /// The depth is increased for every encountered `(` and decreased for every encountered `)`.
        /// The depth cannot be zero (since that would mean that the outer-most pair of brackets has already been closed).
        Brackets(usize /* depth */, String /* buffer */),
    }

    /// The current state of the state machine.
    /// Each individual operation is handled within one state.
    /// Brackets are considered a single state and are handled using recursion.
    /// The state machine starts with adding something, so the initial state is `State::Term(Operation::Add, false, Value::None)`.
    enum State {
        /// An operation has been read. Possibly a value has started being read.
        Term(
            /// The operation of this term.
            Operation,
            /// Whether this term is to be negated.
            bool,
            /// The value of the term, which the operation is applied to.
            /// May be at any state: A complete value, down to a value which hasn't even begun being read.
            Value,
        ),
        /// The previous term was fully processed. Awaiting operation (or brackets, which implicitly multiply).
        AfterTerm,
    }

    // The work-in-progress result. Contains all complete terms added so far.
    let mut result = Term::from(0u32);

    // The current work-in-progress term.
    // Whenever a * or / is encountered, its applied to this term.
    // When a + is encountered, this term is added to the result and replaced with the new term.
    let mut working_term = Box::new([Term::from(0u32)]);

    // Processes a term, applying the operation as appropriate.
    // Multiplications and divisions are applied to the current `working_term`.
    // If the operation is an addition, the current `working_term` is added to the result and replaced by this new term.
    let mut process_term = |operation: Operation, negated: bool, term: Term<u32>| {
        let t = if negated { -term } else { term };
        match operation {
            Operation::Add => {
                result += take(&mut working_term[0]);
                working_term[0] = t;
            }
            Operation::Mul => {
                working_term[0] *= t;
            }
            Operation::Div => {
                working_term[0] /= t;
            }
        }
    };

    // The current state of the state machine.
    // Starts with adding something.
    let mut state = State::Term(Operation::Add, false, Value::None);

    // The state machine
    for char in value.chars() {
        state = match state {
            State::AfterTerm => match char {
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
                            process_term(op, neg, parse_string(&buffer)?);
                            State::AfterTerm
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
                        process_term(op, neg, term);
                        State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                    }
                    '-' => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        process_term(op, neg, term);
                        State::Term(Operation::Add, true, Value::None)
                    }
                    '(' => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        process_term(op, neg, term);
                        State::Term(Operation::Mul, false, Value::Brackets(1, String::new()))
                    }
                    any if any.is_whitespace() => {
                        let term = Term::from(buffer.parse::<u32>().unwrap());
                        process_term(op, neg, term);
                        State::AfterTerm
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
                        process_term(op, neg, term);
                        State::Term(Operation::try_from(char).unwrap(), false, Value::None)
                    }
                    '-' => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        process_term(op, neg, term);
                        State::Term(Operation::Add, true, Value::None)
                    }
                    '(' => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        process_term(op, neg, term);
                        State::Term(Operation::Mul, false, Value::Brackets(1, String::new()))
                    }
                    any if any.is_whitespace() => {
                        let term = Term::from(pre)
                            + Term::div(
                                buffer.parse::<u32>().unwrap(),
                                10u32.pow(buffer.len() as u32),
                            );
                        process_term(op, neg, term);
                        State::AfterTerm
                    }
                    any => return Err(TryFromStrError::UnexpectedCharacter(any)),
                },
            },
        }
    }

    // Processes the final state the machine was left in.
    match state {
        State::Term(op, neg, val) => match val {
            Value::None | Value::Brackets(_, _) => return Err(TryFromStrError::UnexpectedEof),
            Value::PreComma(buffer) => {
                let term = Term::from(buffer.parse::<u32>().unwrap());
                process_term(op, neg, term);
            }
            Value::PostComma(pre, buffer) => {
                let term = Term::from(pre)
                    + Term::div(
                        buffer.parse::<u32>().unwrap(),
                        10u32.pow(buffer.len() as u32),
                    );
                process_term(op, neg, term);
            }
        },
        State::AfterTerm => (),
    }

    result += take(&mut working_term[0]);

    Ok(result)
}
