use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use crate::{
    operation::{
        traits::{Calc, Convert, SetVars},
        variable::Variable,
        Operation,
    },
    parse_string::{parse_string, TryFromStrError},
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

impl Term<u32> {
    /// Optimizes and calculates the term.
    pub fn process<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<u32>,
    >(
        term: &str,
    ) -> Result<Output, TryFromStrError> {
        Ok(Term::try_from(term)?.calc())
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
    > Term<Num>
{
    /// Converts the internal number type.
    pub fn convert<
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + Clone
            + Default
            + PartialOrd
            + From<Num>,
    >(
        self,
    ) -> Term<T> {
        Term {
            operation: self.operation.convert(),
        }
    }

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

    /// Replaces all matching variables with the given term, and calculates the result.
    pub fn use_var<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
        name: &str,
        term: &Term<Num>,
    ) -> Output {
        self.operation.set_vars(&[(name, &term.operation)]).calc()
    }

    /// Replaces all matching variables with the given term.
    pub fn with_var(&self, name: &str, term: &Term<Num>) -> Self {
        Term {
            operation: self.operation.set_vars(&[(name, &term.operation)]),
        }
    }

    /// Replaces all matching variables with the given term.
    pub fn set_var(&mut self, name: &str, term: &Term<Num>) -> &Self {
        self.operation = self.operation.set_vars(&[(name, &term.operation)]);
        self
    }

    /// Replaces all matching variables with the given terms, and calculates the result.
    pub fn use_vars<
        Output: Add<Output = Output>
            + Sub<Output = Output>
            + Mul<Output = Output>
            + Div<Output = Output>
            + Neg<Output = Output>
            + From<Num>,
    >(
        &self,
        variables: &[(&str, &Term<Num>)],
    ) -> Output {
        let vars_as_ops: Vec<(&str, &Operation<Num>)> = variables
            .iter()
            .map(|var| (var.0, &var.1.operation))
            .collect();

        self.operation.set_vars(&vars_as_ops).calc()
    }

    /// Replaces all matching variables with the given terms.
    pub fn with_vars(&self, variables: &[(&str, &Term<Num>)]) -> Self {
        let vars_as_ops: Vec<(&str, &Operation<Num>)> = variables
            .iter()
            .map(|var| (var.0, &var.1.operation))
            .collect();

        Term {
            operation: self.operation.set_vars(&vars_as_ops),
        }
    }

    /// Replaces all matching variables with the given terms.
    pub fn set_vars(&mut self, variables: &[(&str, &Term<Num>)]) -> &Self {
        let vars_as_ops: Vec<(&str, &Operation<Num>)> = variables
            .iter()
            .map(|var| (var.0, &var.1.operation))
            .collect();

        self.operation = self.operation.set_vars(&vars_as_ops);
        self
    }

    /// Creates a new variable.
    pub fn var(name: impl Into<String>) -> Self {
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
