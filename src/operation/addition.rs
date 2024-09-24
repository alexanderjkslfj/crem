use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    division::Division,
    multiplication::Multiplication,
    negation::Negation,
    number::Number,
    traits::{Calc, CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Addition<
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
    > Convert<Num> for Addition<Num>
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
        Operation::Addition(Addition {
            summands: self
                .summands
                .into_iter()
                .map(|summand| summand.convert())
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
    > Addition<Num>
{
    pub fn add_num(&mut self, num: Number<Num>) {
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
