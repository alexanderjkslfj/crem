//! Calculate with minimal precision loss: Terms created using `crem` are automatically simplified, reducing precision loss to a minimum.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod parse_string;

use parse_string::parse_string;
pub use parse_string::ParsingError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A mathematical term.
///
/// The term is simplified before being calculated, minimizing precision loss.
///
/// ```rust
/// # use crem::Term;
/// assert_ne!(0.1 + 0.2, 0.3);
/// assert_eq!((Term::try_from(0.1)? + Term::try_from(0.2)?).calc(), 0.3);
/// # Ok::<(), ()>(())
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Term {
    operation: Operation,
}

impl Term {
    /// Calculates the result of the term.
    pub fn calc(&self) -> f64 {
        self.operation.calc()
    }

    /// Replaces all matching variables with the given term.
    pub fn set_variable(&mut self, name: &str, term: &Term) {
        self.operation = self.operation.set_vars(&[(name, &term.operation)]);
    }

    /// Replaces all matching variables with the given terms.
    pub fn set_variables(&mut self, variables: &[(&str, &Term)]) {
        let vars_as_ops: Vec<(&str, &Operation)> = variables
            .iter()
            .map(|var| (var.0, &var.1.operation))
            .collect();

        self.operation = self.operation.set_vars(&vars_as_ops)
    }

    /// Creates a new variable.
    pub fn new_variable(name: impl Into<String>) -> Self {
        Term {
            operation: Operation::Variable(Variable::from(name.into())),
        }
    }

    /// Creates a division. Simplifies if possible.
    ///
    /// ```rust
    /// # use crem::Term;
    /// assert_eq!(Term::div(2,6), Term::div(1,3));
    /// assert_eq!(Term::div(4,2), Term::from(2));
    /// assert_eq!(Term::div(1,2).calc(), 0.5);
    /// ```
    pub fn div(divident: u32, divisor: u32) -> Self {
        Self::from(divident) / Self::from(divisor)
    }
}

const MAX_U32_AS_F64: f64 = u32::MAX as f64;

fn f64_to_u32(value: f64) -> Result<u32, ()> {
    if value.fract() != 0.0 {
        return Err(());
    }
    if value > MAX_U32_AS_F64 {
        return Err(());
    }
    Ok(value as u32)
}

impl TryFrom<String> for Term {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Term::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Term {
    type Error = ParsingError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Term::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Term {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_string(value)
    }
}

impl TryFrom<f64> for Term {
    type Error = ();

    /// Converts a float to a fraction. For example: `1.1` will be converted to `11/10`.
    ///
    /// It's recommended to use `Term.div` instead. Converting from floats loses precision.
    ///
    /// Conversion can fail if resulting divident is greater than `u32::MAX`.
    fn try_from(mut value: f64) -> Result<Self, Self::Error> {
        if value == 0.0 {
            return Ok(Term::from(0u32));
        }

        let invert = value.is_sign_negative();
        if invert {
            value = -value;
        }

        let mut exponent = 0;

        loop {
            let offset = 10.0f64.powi(exponent);

            let raised = value * offset;

            if raised == raised.trunc() {
                if invert {
                    return Ok(-Term::div(f64_to_u32(raised)?, offset as u32));
                } else {
                    return Ok(Term::div(f64_to_u32(raised)?, offset as u32));
                }
            }

            exponent += 1;
        }
    }
}

const MAX_U32_AS_F32: f32 = u32::MAX as f32;

fn f32_to_u32(value: f32) -> Result<u32, ()> {
    if value.fract() != 0.0 {
        return Err(());
    }
    if value > MAX_U32_AS_F32 {
        return Err(());
    }
    Ok(value as u32)
}

impl TryFrom<f32> for Term {
    type Error = ();

    /// Converts a float to a fraction. For example: `1.1` will be converted to `11/10`.
    ///
    /// It's recommended to use `Term.div` instead. Converting from floats loses precision.
    ///
    /// Conversion can fail if resulting divident is greater than `u32::MAX`.
    fn try_from(mut value: f32) -> Result<Self, Self::Error> {
        if value == 0.0 {
            return Ok(Term::from(0u32));
        }

        let invert = value.is_sign_negative();
        if invert {
            value = -value;
        }

        let mut exponent = 0;

        loop {
            let offset = 10.0f32.powi(exponent);

            let raised = value * offset;

            if raised == raised.trunc() {
                if invert {
                    return Ok(-Term::div(f32_to_u32(raised)?, offset as u32));
                } else {
                    return Ok(Term::div(f32_to_u32(raised)?, offset as u32));
                }
            }

            exponent += 1;
        }
    }
}

impl From<u32> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `3` results in the term `3`.
    fn from(value: u32) -> Self {
        Term {
            operation: Operation::from(value),
        }
    }
}

impl From<u16> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `3` results in the term `3`.
    fn from(value: u16) -> Self {
        Term::from(u32::from(value))
    }
}

impl From<u8> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `3` results in the term `3`.
    fn from(value: u8) -> Self {
        Term::from(u32::from(value))
    }
}

impl From<i32> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `-3` results in the term `-3`.
    fn from(value: i32) -> Self {
        if value.is_negative() {
            Term {
                operation: -Operation::from(value.abs() as u32),
            }
        } else {
            Term {
                operation: Operation::from(value.abs() as u32),
            }
        }
    }
}

impl From<i16> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `-3` results in the term `-3`.
    fn from(value: i16) -> Self {
        Term::from(i32::from(value))
    }
}

impl From<i8> for Term {
    /// Creates a Term consisting of only the number.
    ///
    /// Example: Entering `-3` results in the term `-3`.
    fn from(value: i8) -> Self {
        Term::from(i32::from(value))
    }
}

impl Default for Term {
    /// Returns the default Term: `0`
    fn default() -> Self {
        Term {
            operation: Operation::default(),
        }
    }
}

impl AddAssign for Term {
    fn add_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) + rhs.operation;
    }
}

impl Add for Term {
    type Output = Term;

    fn add(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation + rhs.operation,
        }
    }
}

impl SubAssign for Term {
    fn sub_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) - rhs.operation;
    }
}

impl Sub for Term {
    type Output = Term;

    fn sub(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation - rhs.operation,
        }
    }
}

impl MulAssign for Term {
    fn mul_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) * rhs.operation;
    }
}

impl Mul for Term {
    type Output = Term;

    fn mul(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation * rhs.operation,
        }
    }
}

impl DivAssign for Term {
    fn div_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) / rhs.operation;
    }
}

impl Div for Term {
    type Output = Term;

    fn div(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation / rhs.operation,
        }
    }
}

impl Neg for Term {
    type Output = Term;

    fn neg(self) -> Self::Output {
        Term {
            operation: -self.operation,
        }
    }
}

trait Calc {
    fn calc(&self) -> f64;
}

trait SetVars {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation;
}

trait CanAddNumWell {
    fn can_add_number_well(&self) -> bool;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Operation {
    Addition(Addition),
    Multiplication(Multiplication),
    Division(Division),
    Negation(Negation),
    Number(Number),
    Variable(Variable),
}

impl CanAddNumWell for Operation {
    fn can_add_number_well(&self) -> bool {
        match self {
            Operation::Addition(add) => add.can_add_number_well(),
            Operation::Multiplication(mul) => mul.can_add_number_well(),
            Operation::Division(div) => div.can_add_number_well(),
            Operation::Negation(neg) => neg.can_add_number_well(),
            Operation::Number(num) => num.can_add_number_well(),
            Operation::Variable(var) => var.can_add_number_well(),
        }
    }
}

impl SetVars for Operation {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        match self {
            Operation::Addition(add) => add.set_vars(vars),
            Operation::Multiplication(mul) => mul.set_vars(vars),
            Operation::Division(div) => div.set_vars(vars),
            Operation::Negation(neg) => neg.set_vars(vars),
            Operation::Number(num) => num.set_vars(vars),
            Operation::Variable(var) => var.set_vars(vars),
        }
    }
}

impl Calc for Operation {
    fn calc(&self) -> f64 {
        match self {
            Operation::Addition(add) => add.calc(),
            Operation::Multiplication(mul) => mul.calc(),
            Operation::Division(div) => div.calc(),
            Operation::Negation(inv) => inv.calc(),
            Operation::Number(num) => num.calc(),
            Operation::Variable(_) => panic!("Cannot calculate result of a term with variables."),
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Number(Number::default())
    }
}

impl From<u32> for Operation {
    fn from(value: u32) -> Self {
        Operation::Number(Number { value })
    }
}

impl Add for Operation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first + second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first + second,
            (Operation::Division(first), Operation::Division(second)) => first + second,
            (Operation::Negation(first), Operation::Negation(second)) => first + second,
            (Operation::Number(first), Operation::Number(second)) => first + second,
            (Operation::Variable(first), Operation::Variable(second)) => first + second,

            (Operation::Number(num), any) if (num.value == 0) => any,
            (any, Operation::Number(num)) if (num.value == 0) => any,

            (Operation::Number(num), Operation::Addition(mut add)) => {
                add.add_num(num);
                Operation::Addition(add)
            }
            (Operation::Addition(mut add), Operation::Number(num)) => {
                add.add_num(num);
                Operation::Addition(add)
            }

            (Operation::Negation(neg), any) => any - (*neg.value),
            (any, Operation::Negation(neg)) => any - (*neg.value),

            (Operation::Addition(mut add), any) => {
                add.summands.push(any);
                Operation::Addition(add)
            }
            (any, Operation::Addition(mut add)) => {
                add.summands.push(any);
                Operation::Addition(add)
            }

            // experimental
            (Operation::Division(div), any) => {
                (any * (*div.divisor).clone() + (*div.divident)) / (*div.divisor)
            }
            (any, Operation::Division(div)) => {
                (any * (*div.divisor).clone() + (*div.divident)) / (*div.divisor)
            }

            // NOTE: match with default
            (first, second) => Operation::Addition(Addition {
                summands: vec![first, second],
            }),
        }
    }
}

impl Div for Operation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(divident), Operation::Addition(divisor)) => divident / divisor,
            (Operation::Multiplication(divident), Operation::Multiplication(divisor)) => {
                divident / divisor
            }
            (Operation::Division(divident), Operation::Division(divisor)) => divident / divisor,
            (Operation::Negation(divident), Operation::Negation(divisor)) => divident / divisor,
            (Operation::Number(divident), Operation::Number(divisor)) => divident / divisor,
            (Operation::Variable(divident), Operation::Variable(divisor)) => divident / divisor,

            (_, Operation::Number(num)) if (num.value == 0) => panic!("Cannot divide by zero."),
            (any, Operation::Number(num)) if (num.value == 1) => any,
            (Operation::Number(num), _) if (num.value == 0) => Operation::Number(num),

            (Operation::Negation(neg), any) => -((*neg.value) / any),
            (any, Operation::Negation(neg)) => -(any / (*neg.value)),

            (any, Operation::Division(div)) => any * ((*div.divisor) / (*div.divident)),
            (Operation::Division(div), any) => {
                Operation::from(1) / (any * ((*div.divisor) / (*div.divident)))
            }

            // NOTE: match with default
            (divident, divisor) => Operation::Division(Division {
                divident: Box::new(divident),
                divisor: Box::new(divisor),
            }),
        }
    }
}

impl Mul for Operation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first * second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first * second,
            (Operation::Division(first), Operation::Division(second)) => first * second,
            (Operation::Negation(first), Operation::Negation(second)) => first * second,
            (Operation::Number(first), Operation::Number(second)) => first * second,
            (Operation::Variable(first), Operation::Variable(second)) => first * second,

            (Operation::Number(num), _) if (num.value == 0) => Operation::Number(num),
            (_, Operation::Number(num)) if (num.value == 0) => Operation::Number(num),
            (Operation::Number(num), any) if (num.value == 1) => any,
            (any, Operation::Number(num)) if (num.value == 1) => any,

            (any, Operation::Negation(neg)) => -(any * (*neg.value)),
            (Operation::Negation(neg), any) => -((*neg.value) * any),

            (any, Operation::Division(div)) => (any * (*div.divident)) / (*div.divisor),
            (Operation::Division(div), any) => (any * (*div.divident)) / (*div.divisor),

            (Operation::Multiplication(mut mul), any) => {
                mul.multipliers.push(any);
                Operation::Multiplication(mul)
            }
            (any, Operation::Multiplication(mut mul)) => {
                mul.multipliers.push(any);
                Operation::Multiplication(mul)
            }

            // NOTE: match with default
            (first, second) => Operation::Multiplication(Multiplication {
                multipliers: vec![first, second],
            }),
        }
    }
}

impl Sub for Operation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first - second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first - second,
            (Operation::Division(first), Operation::Division(second)) => first - second,
            (Operation::Negation(first), Operation::Negation(second)) => first - second,
            (Operation::Number(first), Operation::Number(second)) => first - second,
            (Operation::Variable(first), Operation::Variable(second)) => first - second,

            (Operation::Number(num), any) if (num.value == 0) => -any,
            (any, Operation::Number(num)) if (num.value == 0) => any,

            (Operation::Negation(neg), any) => -((*neg.value) + any),
            (any, Operation::Negation(neg)) => any + (*neg.value),

            // NOTE: match with default
            (first, second) => Operation::Addition(Addition {
                summands: vec![
                    first,
                    Operation::Negation(Negation {
                        value: Box::new(second),
                    }),
                ],
            }),
        }
    }
}

impl Neg for Operation {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        match self {
            Operation::Addition(add) => -add,
            Operation::Multiplication(mul) => -mul,
            Operation::Division(div) => -div,
            Operation::Negation(neg) => -neg,
            Operation::Number(num) => -num,
            Operation::Variable(var) => -var,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Negation {
    pub value: Box<Operation>,
}

impl CanAddNumWell for Negation {
    fn can_add_number_well(&self) -> bool {
        self.value.can_add_number_well()
    }
}

impl SetVars for Negation {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        -self.value.set_vars(vars)
    }
}

impl Calc for Negation {
    fn calc(&self) -> f64 {
        -self.value.calc()
    }
}

impl Add for Negation {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new((*self.value) + (*rhs.value)),
        })
    }
}

impl Div for Negation {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        (*self.value) / (*rhs.value)
    }
}

impl Mul for Negation {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        (*self.value) * (*rhs.value)
    }
}

impl Sub for Negation {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        (*rhs.value) - (*self.value)
    }
}

impl Neg for Negation {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        *self.value
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Addition {
    pub summands: Vec<Operation>,
}

impl Addition {
    fn add_num(&mut self, num: Number) {
        for i in 0..self.summands.len() {
            if self.summands[i].can_add_number_well() {
                let added_summand = self.summands.remove(i) + Operation::Number(num);
                self.summands.push(added_summand);
                return;
            }
        }
        self.summands.push(Operation::Number(num))
    }
}

impl CanAddNumWell for Addition {
    fn can_add_number_well(&self) -> bool {
        for summand in &self.summands {
            if summand.can_add_number_well() {
                return true;
            }
        }
        false
    }
}

impl SetVars for Addition {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        self.summands
            .iter()
            .fold(Operation::from(0u32), |acc, op| acc + op.set_vars(vars))
    }
}

impl Calc for Addition {
    fn calc(&self) -> f64 {
        self.summands.iter().map(|summand| summand.calc()).sum()
    }
}

impl Add for Addition {
    type Output = Operation;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.summands.append(&mut rhs.summands);
        Operation::Addition(Addition {
            summands: self.summands,
        })
    }
}

impl Mul for Addition {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multipliers: vec![Operation::Addition(self), Operation::Addition(rhs)],
        })
    }
}

impl Div for Addition {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Addition(self)),
            divisor: Box::new(Operation::Addition(rhs)),
        })
    }
}

impl Sub for Addition {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO: optimize
        Operation::Addition(Addition {
            summands: self
                .summands
                .into_iter()
                .chain(rhs.summands.into_iter().map(|summand| -summand))
                .collect(),
        })
    }
}

impl Neg for Addition {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Addition(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Division {
    pub divident: Box<Operation>,
    pub divisor: Box<Operation>,
}

impl CanAddNumWell for Division {
    fn can_add_number_well(&self) -> bool {
        match *self.divisor {
            Operation::Number(_) => self.divident.can_add_number_well(),
            _ => false,
        }
    }
}

impl SetVars for Division {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        self.divident.set_vars(vars) / self.divisor.set_vars(vars)
    }
}

impl Calc for Division {
    fn calc(&self) -> f64 {
        self.divident.calc() / self.divisor.calc()
    }
}

impl Add for Division {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        if self.divisor == rhs.divisor {
            (*self.divident + *rhs.divident) / *self.divisor
        } else {
            let s_divident = *self.divident;
            let r_divident = *rhs.divident;
            let s_divisor = *self.divisor;
            let r_divisor = *rhs.divisor;

            ((s_divident * r_divisor.clone()) + (r_divident * s_divisor.clone()))
                / (s_divisor * r_divisor)
        }
    }
}

impl Mul for Division {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        ((*self.divident) * (*rhs.divident)) / ((*self.divisor) * (*rhs.divisor))
    }
}

impl Div for Division {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(self) * ((*rhs.divisor) / (*rhs.divident))
    }
}

impl Sub for Division {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        let s_divident = *self.divident;
        let s_divisor = *self.divisor;
        let r_divident = *rhs.divident;
        let r_divisor = *rhs.divisor;

        if s_divisor == r_divisor {
            (s_divident - r_divident) / s_divisor
        } else {
            ((s_divident * r_divisor.clone()) + (r_divident * s_divisor.clone()))
                / (s_divisor * r_divisor)
        }
    }
}

impl Neg for Division {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Division(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Multiplication {
    multipliers: Vec<Operation>,
}

impl CanAddNumWell for Multiplication {
    fn can_add_number_well(&self) -> bool {
        false
    }
}

impl SetVars for Multiplication {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        self.multipliers
            .iter()
            .fold(Operation::from(1u32), |acc, op| acc * op.set_vars(vars))
    }
}

impl Calc for Multiplication {
    fn calc(&self) -> f64 {
        self.multipliers
            .iter()
            .map(|multiplier| multiplier.calc())
            .product()
    }
}

impl Add for Multiplication {
    type Output = Operation;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut on_both_sides = Vec::new();

        // TODO: optimize
        for i in (0..self.multipliers.len()).rev() {
            for j in (i..rhs.multipliers.len()).rev() {
                if self.multipliers[i] == rhs.multipliers[j] {
                    on_both_sides.push(self.multipliers.remove(i));
                    rhs.multipliers.remove(j);
                }
            }
        }

        if on_both_sides.is_empty() {
            Operation::Addition(Addition {
                summands: vec![
                    Operation::Multiplication(self),
                    Operation::Multiplication(rhs),
                ],
            })
        } else {
            on_both_sides.push(Operation::Addition(Addition {
                summands: vec![
                    Operation::Multiplication(Multiplication {
                        multipliers: self.multipliers,
                    }),
                    Operation::Multiplication(Multiplication {
                        multipliers: rhs.multipliers,
                    }),
                ],
            }));
            Operation::Multiplication(Multiplication {
                multipliers: on_both_sides,
            })
        }
    }
}

impl Mul for Multiplication {
    type Output = Operation;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.multipliers.append(&mut rhs.multipliers);
        Operation::Multiplication(Multiplication {
            multipliers: self.multipliers,
        })
    }
}

impl Div for Multiplication {
    type Output = Operation;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        for i in (0..self.multipliers.len()).rev() {
            for j in (i..rhs.multipliers.len()).rev() {
                if self.multipliers[i] == rhs.multipliers[j] {
                    self.multipliers.remove(i);
                    rhs.multipliers.remove(j);
                }
            }
        }
        Operation::Division(Division {
            divident: Box::new(Operation::Multiplication(self)),
            divisor: Box::new(Operation::Multiplication(rhs)),
        })
    }
}

impl Sub for Multiplication {
    type Output = Operation;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        let mut on_both_sides = Vec::new();

        // TODO: optimize
        for i in (0..self.multipliers.len()).rev() {
            for j in (i..rhs.multipliers.len()).rev() {
                if self.multipliers[i] == rhs.multipliers[j] {
                    on_both_sides.push(self.multipliers.remove(i));
                    rhs.multipliers.remove(j);
                }
            }
        }

        if on_both_sides.is_empty() {
            Operation::Addition(Addition {
                summands: vec![Operation::Multiplication(self), -rhs],
            })
        } else {
            on_both_sides.push(Operation::Addition(Addition {
                summands: vec![
                    Operation::Multiplication(Multiplication {
                        multipliers: self.multipliers,
                    }),
                    -Operation::Multiplication(Multiplication {
                        multipliers: rhs.multipliers,
                    }),
                ],
            }));
            Operation::Multiplication(Multiplication {
                multipliers: on_both_sides,
            })
        }
    }
}

impl Neg for Multiplication {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Multiplication(self)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
struct Number {
    value: u32,
}

impl CanAddNumWell for Number {
    fn can_add_number_well(&self) -> bool {
        true
    }
}

impl SetVars for Number {
    fn set_vars(&self, _vars: &[(&str, &Operation)]) -> Operation {
        Operation::Number(self.clone())
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number { value }
    }
}

impl Calc for Number {
    fn calc(&self) -> f64 {
        f64::from(self.value)
    }
}

impl Add for Number {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value + rhs.value))
    }
}

impl Mul for Number {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value * rhs.value))
    }
}

impl Div for Number {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        if self.value % rhs.value == 0 {
            Operation::Number(Number::from(self.value / rhs.value))
        } else {
            let gcd = greatest_common_divisor(self.value, rhs.value);
            if gcd == 1 {
                Operation::Division(Division {
                    divident: Box::new(Operation::Number(self)),
                    divisor: Box::new(Operation::Number(rhs)),
                })
            } else {
                Operation::Division(Division {
                    divident: Box::new(Operation::from(self.value / gcd)),
                    divisor: Box::new(Operation::from(rhs.value / gcd)),
                })
            }
        }
    }
}

impl Sub for Number {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.value < rhs.value {
            -Operation::from(rhs.value - self.value)
        } else {
            Operation::from(self.value - rhs.value)
        }
    }
}

impl Neg for Number {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        if self.value == 0 {
            Operation::Number(self)
        } else {
            Operation::Negation(Negation {
                value: Box::new(Operation::Number(self)),
            })
        }
    }
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    // euclidean algorithm

    let (mut smaller, mut bigger) = if a < b { (a, b) } else { (b, a) };

    while smaller != 0 {
        (smaller, bigger) = (bigger % smaller, smaller);
    }

    bigger
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
struct Variable {
    name: String,
}

impl CanAddNumWell for Variable {
    fn can_add_number_well(&self) -> bool {
        false
    }
}

impl SetVars for Variable {
    fn set_vars(&self, vars: &[(&str, &Operation)]) -> Operation {
        for var in vars {
            if self.name == var.0 {
                return var.1.clone();
            }
        }
        Operation::Variable(self.clone())
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Variable { name: value }
    }
}

impl Add for Variable {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        if self.name == rhs.name {
            Operation::Multiplication(Multiplication {
                multipliers: vec![Operation::from(2u32), Operation::Variable(self)],
            })
        } else {
            Operation::Addition(Addition {
                summands: vec![Operation::Variable(self), Operation::Variable(rhs)],
            })
        }
    }
}

impl Mul for Variable {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multipliers: vec![Operation::Variable(self), Operation::Variable(rhs)],
        })
    }
}

impl Div for Variable {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Variable(self)),
            divisor: Box::new(Operation::Variable(rhs)),
        })
    }
}

impl Sub for Variable {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Operation::from(0u32)
        } else {
            Operation::Addition(Addition {
                summands: vec![Operation::Variable(self), -Operation::Variable(rhs)],
            })
        }
    }
}

impl Neg for Variable {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Variable(self)),
        })
    }
}
