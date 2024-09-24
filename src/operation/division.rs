use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    negation::Negation,
    traits::{Calc, CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Division<
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
    > Convert<Num> for Division<Num>
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
        Operation::Division(Division {
            divident: Box::new(self.divident.convert()),
            divisor: Box::new(self.divisor.convert()),
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
