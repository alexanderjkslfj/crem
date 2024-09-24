use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub mod traits;

mod addition;
mod division;
mod multiplication;
mod negation;
mod number;
pub mod variable;

use addition::Addition;
use division::Division;
use multiplication::Multiplication;
use negation::Negation;
use number::Number;
use traits::{Calc, CanAddNumWell, Convert, SetVars};
use variable::Variable;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Operation<
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
    > Convert<Num> for Operation<Num>
{
    fn convert<
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
    ) -> Operation<T> {
        match self {
            Self::Addition(add) => add.convert(),
            Self::Multiplication(mul) => mul.convert(),
            Self::Division(div) => div.convert(),
            Self::Negation(neg) => neg.convert(),
            Self::Number(num) => num.convert(),
            Self::Variable(var) => var.convert(),
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
