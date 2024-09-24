use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    division::Division,
    negation::Negation,
    traits::{CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Number<
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
    pub value: Num,
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
    > Convert<Num> for Number<Num>
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
        Operation::Number(Number {
            value: T::from(self.value),
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
