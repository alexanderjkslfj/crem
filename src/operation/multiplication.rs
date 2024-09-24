use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    addition::Addition,
    division::Division,
    negation::Negation,
    traits::{Calc, CanAddNumWell, Convert, SetVars},
    Operation,
};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Multiplication<
    Num: Add<Output = Num>
        + Sub<Output = Num>
        + Mul<Output = Num>
        + Div<Output = Num>
        + Rem<Output = Num>
        + Clone
        + Default
        + PartialOrd,
> {
    pub multipliers: Vec<Operation<Num>>,
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
    > Convert<Num> for Multiplication<Num>
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
        Operation::Multiplication(Multiplication {
            multipliers: self
                .multipliers
                .into_iter()
                .map(|multiplier| multiplier.convert())
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
