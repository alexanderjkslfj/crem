use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use super::{
    addition::Addition,
    division::Division,
    multiplication::Multiplication,
    negation::Negation,
    traits::{CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
pub struct Variable<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    pub phantom: PhantomData<Num>,
    pub name: String,
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
    > Convert<Num> for Variable<Num>
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
        Operation::Variable(Variable {
            phantom: PhantomData::default(),
            name: self.name,
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
