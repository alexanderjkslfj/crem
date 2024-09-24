use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    traits::{Calc, CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Negation<
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
    > Convert<Num> for Negation<Num>
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
        Operation::Negation(Negation {
            value: Box::new(self.value.convert()),
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
