//! Calculate with minimal precision loss: Terms created using `crem` are automatically simplified, reducing precision loss to a minimum.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod parse_string;

use parse_string::parse_string;
pub use parse_string::TryFromStrError;
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

/// A mathematical term.
///
/// The term is simplified before being calculated, minimizing precision loss.
///
/// ```rust
/// # use crem::*;
/// assert_ne!(0.1 + 0.2, 0.3);
/// assert_eq!(Term::try_from("0.1 + 0.2")?.calc::<f64>(), 0.3);
/// # Ok::<(), TryFromStrError>(())
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Term<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    operation: Operation<Num>,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Term<Num>
{
    /// Calculates the result of the term.
    pub fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        self.operation.calc()
    }

    /// Replaces all matching variables with the given term.
    pub fn set_variable(&mut self, name: &str, term: &Term<Num>) {
        self.operation = self.operation.set_vars(&[(name, &term.operation)]);
    }

    /// Replaces all matching variables with the given terms.
    pub fn set_variables(&mut self, variables: &[(&str, &Term<Num>)]) {
        let vars_as_ops: Vec<(&str, &Operation<Num>)> = variables
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
    /// assert_eq!(Term::div(1,2).calc::<f64>(), 0.5);
    /// ```
    pub fn div(divident: Num, divisor: Num) -> Self {
        Self::from(divident) / Self::from(divisor)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > From<Num> for Term<Num>
{
    fn from(value: Num) -> Self {
        Term {
            operation: Operation::from(value),
        }
    }
}

impl TryFrom<String> for Term<u32> {
    type Error = TryFromStrError;

    /// Performs the conversion.
    ///
    /// ```rust
    /// # use crem::*;
    /// assert_eq!(Term::try_from("7")?, Term::from(7));
    /// assert_eq!(Term::try_from("8 / 2")?, Term::from(4));
    /// assert_eq!(Term::try_from("1.3 + 3.7")?, Term::from(5));
    /// assert_eq!(Term::try_from("3(8-8/2)")?, Term::from(12));
    /// # Ok::<(), TryFromStrError>(())
    /// ```
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Term::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Term<u32> {
    type Error = TryFromStrError;

    /// Performs the conversion.
    ///
    /// ```rust
    /// # use crem::*;
    /// assert_eq!(Term::try_from("7")?, Term::from(7));
    /// assert_eq!(Term::try_from("8 / 2")?, Term::from(4));
    /// assert_eq!(Term::try_from("1.3 + 3.7")?, Term::from(5));
    /// assert_eq!(Term::try_from("3(8-8/2)")?, Term::from(12));
    /// # Ok::<(), TryFromStrError>(())
    /// ```
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Term::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Term<u32> {
    type Error = TryFromStrError;

    /// Performs the conversion.
    ///
    /// ```rust
    /// # use crem::*;
    /// assert_eq!(Term::try_from("7")?, Term::from(7));
    /// assert_eq!(Term::try_from("8 / 2")?, Term::from(4));
    /// assert_eq!(Term::try_from("1.3 + 3.7")?, Term::from(5));
    /// assert_eq!(Term::try_from("3(8-8/2)")?, Term::from(12));
    /// # Ok::<(), TryFromStrError>(())
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_string(value)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Default for Term<Num>
{
    /// Returns the default Term: `0`
    fn default() -> Self {
        Term {
            operation: Operation::default(),
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > AddAssign for Term<Num>
{
    fn add_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) + rhs.operation;
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Term<Num>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation + rhs.operation,
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SubAssign for Term<Num>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) - rhs.operation;
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Term<Num>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation - rhs.operation,
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > MulAssign for Term<Num>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) * rhs.operation;
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Term<Num>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation * rhs.operation,
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > DivAssign for Term<Num>
{
    fn div_assign(&mut self, rhs: Self) {
        self.operation = std::mem::take(&mut self.operation) / rhs.operation;
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Term<Num>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Term {
            operation: self.operation / rhs.operation,
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Term<Num>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Term {
            operation: -self.operation,
        }
    }
}

trait Calc<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output;
}

trait SetVars<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num>;
}

trait CanAddNumWell {
    fn can_add_number_well(&self) -> bool;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Operation<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    Addition(Addition<Num>),
    Multiplication(Multiplication<Num>),
    Division(Division<Num>),
    Negation(Negation<Num>),
    Number(Number<Num>),
    Variable(Variable<Num>),
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Operation<Num>
{
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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Operation<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Calc<Num> for Operation<Num>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        match self {
            Operation::Addition(add) => add.calc(),
            Operation::Multiplication(mul) => mul.calc(),
            Operation::Division(div) => div.calc(),
            Operation::Negation(inv) => inv.calc(),
            Operation::Number(num) => Output::from(num.value.clone()),
            Operation::Variable(_) => panic!("Cannot calculate result of a term with variables."),
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Default for Operation<Num>
{
    fn default() -> Self {
        Operation::Number(Number::default())
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > From<Num> for Operation<Num>
{
    fn from(value: Num) -> Self {
        Operation::Number(Number { value })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Operation<Num>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first + second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first + second,
            (Operation::Division(first), Operation::Division(second)) => first + second,
            (Operation::Negation(first), Operation::Negation(second)) => first + second,
            (Operation::Number(first), Operation::Number(second)) => first + second,
            (Operation::Variable(first), Operation::Variable(second)) => first + second,

            (Operation::Number(num), any) if (num.value == Num::default()) => any,
            (any, Operation::Number(num)) if (num.value == Num::default()) => any,

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Operation<Num>
{
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

            (_, Operation::Number(num)) if (num.value == Num::default()) => {
                panic!("Cannot divide by zero.")
            }
            (Operation::Number(num), _) if (num.value == Num::default()) => Operation::Number(num),

            // TODO: make generic
            // (any, Operation::Number(num)) if (num.value == 1) => any,
            (Operation::Negation(neg), any) => -((*neg.value) / any),
            (any, Operation::Negation(neg)) => -(any / (*neg.value)),

            (any, Operation::Division(div)) => any * ((*div.divisor) / (*div.divident)),
            (Operation::Division(div), any) => (*div.divident) / ((*div.divisor) * any),

            // NOTE: match with default
            (divident, divisor) => Operation::Division(Division {
                divident: Box::new(divident),
                divisor: Box::new(divisor),
            }),
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Operation<Num>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first * second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first * second,
            (Operation::Division(first), Operation::Division(second)) => first * second,
            (Operation::Negation(first), Operation::Negation(second)) => first * second,
            (Operation::Number(first), Operation::Number(second)) => first * second,
            (Operation::Variable(first), Operation::Variable(second)) => first * second,

            (Operation::Number(num), _) if (num.value == Num::default()) => Operation::Number(num),
            (_, Operation::Number(num)) if (num.value == Num::default()) => Operation::Number(num),

            // TODO: make generic
            // (Operation::Number(num), any) if (num.value == 1) => any,
            // (any, Operation::Number(num)) if (num.value == 1) => any,
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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Operation<Num>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first - second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first - second,
            (Operation::Division(first), Operation::Division(second)) => first - second,
            (Operation::Negation(first), Operation::Negation(second)) => first - second,
            (Operation::Number(first), Operation::Number(second)) => first - second,
            (Operation::Variable(first), Operation::Variable(second)) => first - second,

            (Operation::Number(num), any) if (num.value == Num::default()) => -any,
            (any, Operation::Number(num)) if (num.value == Num::default()) => any,

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Operation<Num>
{
    type Output = Self;

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
struct Negation<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    pub value: Box<Operation<Num>>,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Negation<Num>
{
    fn can_add_number_well(&self) -> bool {
        self.value.can_add_number_well()
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Negation<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        -self.value.set_vars(vars)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Calc<Num> for Negation<Num>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        -self.value.calc::<Output>()
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Negation<Num>
{
    type Output = Operation<Num>;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new((*self.value) + (*rhs.value)),
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Negation<Num>
{
    type Output = Operation<Num>;

    fn div(self, rhs: Self) -> Self::Output {
        (*self.value) / (*rhs.value)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Negation<Num>
{
    type Output = Operation<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        (*self.value) * (*rhs.value)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Negation<Num>
{
    type Output = Operation<Num>;

    fn sub(self, rhs: Self) -> Self::Output {
        (*rhs.value) - (*self.value)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Negation<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        *self.value
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Addition<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    pub summands: Vec<Operation<Num>>,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Addition<Num>
{
    fn add_num(&mut self, num: Number<Num>) {
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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Addition<Num>
{
    fn can_add_number_well(&self) -> bool {
        for summand in &self.summands {
            if summand.can_add_number_well() {
                return true;
            }
        }
        false
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Addition<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        self.summands
            .iter()
            .fold(Operation::from(Num::default()), |acc, op| {
                acc + op.set_vars(vars)
            })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Calc<Num> for Addition<Num>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        let mut result = self.summands[0].calc();
        for i in 1..self.summands.len() {
            result = result + self.summands[i].calc();
        }
        result
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Addition<Num>
{
    type Output = Operation<Num>;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.summands.append(&mut rhs.summands);
        Operation::Addition(Addition {
            summands: self.summands,
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Addition<Num>
{
    type Output = Operation<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multipliers: vec![Operation::Addition(self), Operation::Addition(rhs)],
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Addition<Num>
{
    type Output = Operation<Num>;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Addition(self)),
            divisor: Box::new(Operation::Addition(rhs)),
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Addition<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Addition<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Addition(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Division<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    pub divident: Box<Operation<Num>>,
    pub divisor: Box<Operation<Num>>,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Division<Num>
{
    fn can_add_number_well(&self) -> bool {
        match *self.divisor {
            Operation::Number(_) => self.divident.can_add_number_well(),
            _ => false,
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Division<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        self.divident.set_vars(vars) / self.divisor.set_vars(vars)
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Calc<Num> for Division<Num>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        self.divident.calc::<Output>() / self.divisor.calc::<Output>()
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Division<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Division<Num>
{
    type Output = Operation<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        ((*self.divident) * (*rhs.divident)) / ((*self.divisor) * (*rhs.divisor))
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Division<Num>
{
    type Output = Operation<Num>;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(self) * ((*rhs.divisor) / (*rhs.divident))
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Division<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Division<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Division(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Multiplication<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    multipliers: Vec<Operation<Num>>,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Multiplication<Num>
{
    fn can_add_number_well(&self) -> bool {
        false
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Multiplication<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        let mut result = self.multipliers[0].set_vars(vars);
        for i in 1..self.multipliers.len() {
            result = result * self.multipliers[i].set_vars(vars);
        }
        result
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Calc<Num> for Multiplication<Num>
{
    fn calc<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
    ) -> Output {
        let mut result = self.multipliers[0].calc();
        for i in 1..self.multipliers.len() {
            result = result * self.multipliers[i].calc();
        }
        result
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Multiplication<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Multiplication<Num>
{
    type Output = Operation<Num>;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.multipliers.append(&mut rhs.multipliers);
        Operation::Multiplication(Multiplication {
            multipliers: self.multipliers,
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Multiplication<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Multiplication<Num>
{
    type Output = Operation<Num>;

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

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Multiplication<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Multiplication(self)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
struct Number<
    Num: Sized
        + Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    value: Num,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Number<Num>
{
    fn can_add_number_well(&self) -> bool {
        true
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Number<Num>
{
    fn set_vars(&self, _vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        Operation::Number(self.clone())
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > From<Num> for Number<Num>
{
    fn from(value: Num) -> Self {
        Number { value }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Number<Num>
{
    type Output = Operation<Num>;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value + rhs.value))
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Number<Num>
{
    type Output = Operation<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value * rhs.value))
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Number<Num>
{
    type Output = Operation<Num>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.value.clone() % rhs.value.clone() == Num::default() {
            Operation::Number(Number::from(self.value / rhs.value))
        } else {
            let gcd = greatest_common_divisor(self.value.clone(), rhs.value.clone());
            Operation::Division(Division {
                divident: Box::new(Operation::from(self.value / gcd.clone())),
                divisor: Box::new(Operation::from(rhs.value / gcd)),
            })
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Number<Num>
{
    type Output = Operation<Num>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.value < rhs.value {
            -Operation::from(rhs.value - self.value)
        } else {
            Operation::from(self.value - rhs.value)
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Number<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        if self.value == Num::default() {
            Operation::Number(self)
        } else {
            Operation::Negation(Negation {
                value: Box::new(Operation::Number(self)),
            })
        }
    }
}

fn greatest_common_divisor<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
>(
    a: Num,
    b: Num,
) -> Num {
    // euclidean algorithm

    let (mut smaller, mut bigger) = if a < b { (a, b) } else { (b, a) };

    while smaller != Num::default() {
        let new_bigger = smaller.clone();
        smaller = bigger % smaller;
        bigger = new_bigger;
    }

    bigger
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
struct Variable<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    phantom: PhantomData<Num>,
    name: String,
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > CanAddNumWell for Variable<Num>
{
    fn can_add_number_well(&self) -> bool {
        false
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > SetVars<Num> for Variable<Num>
{
    fn set_vars(&self, vars: &[(&str, &Operation<Num>)]) -> Operation<Num> {
        for var in vars {
            if self.name == var.0 {
                return var.1.clone();
            }
        }
        Operation::Variable(self.clone())
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > From<String> for Variable<Num>
{
    fn from(value: String) -> Self {
        Variable {
            name: value,
            phantom: PhantomData::default(),
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Add for Variable<Num>
{
    type Output = Operation<Num>;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Addition(Addition {
            summands: vec![Operation::Variable(self), Operation::Variable(rhs)],
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Mul for Variable<Num>
{
    type Output = Operation<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multipliers: vec![
                Operation::Variable::<Num>(self),
                Operation::Variable::<Num>(rhs),
            ],
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Div for Variable<Num>
{
    type Output = Operation<Num>;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Variable::<Num>(self)),
            divisor: Box::new(Operation::Variable::<Num>(rhs)),
        })
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Sub for Variable<Num>
{
    type Output = Operation<Num>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Operation::default()
        } else {
            Operation::Addition(Addition {
                summands: vec![Operation::Variable(self), -Operation::Variable(rhs)],
            })
        }
    }
}

impl<
        Num: Add<Output = Num>
            + Sub<Output = Num>
            + Mul<Output = Num>
            + Div<Output = Num>
            + Rem<Output = Num>
            + Clone
            + Default
            + PartialOrd,
    > Neg for Variable<Num>
{
    type Output = Operation<Num>;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Variable::<Num>(self)),
        })
    }
}
