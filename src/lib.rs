use core::ops::{Add, Div, Mul};

pub trait Calc {
    fn calc(&self) -> f64;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Op {
    Addition(Addition),
    Multiplication(Multiplication),
    Division(Division),
    Number(Integer),
}

impl Op {
    pub fn add(first: u32, second: u32) -> Op {
        Op::from(first + second)
    }
    pub fn mul(multiplier: u32, multiplicant: u32) -> Op {
        Op::from(multiplier * multiplicant)
    }
    pub fn div(divident: u32, divisor: u32) -> Op {
        Op::from(divident) / Op::from(divisor)
    }
}

impl Calc for Op {
    fn calc(&self) -> f64 {
        match self {
            Op::Addition(add) => add.calc(),
            Op::Multiplication(mul) => mul.calc(),
            Op::Division(div) => div.calc(),
            Op::Number(num) => num.calc(),
        }
    }
}

impl Default for Op {
    fn default() -> Self {
        Op::Number(Integer::default())
    }
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        Op::Number(Integer { value })
    }
}

impl Add for Op {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Op::Addition(first), Op::Addition(second)) => first + second,
            (Op::Multiplication(first), Op::Multiplication(second)) => first + second,
            (Op::Division(first), Op::Division(second)) => first + second,
            (Op::Number(first), Op::Number(second)) => first + second,
            (first, second) => Op::Addition(Addition {
                first_summand: Box::new(first),
                second_summand: Box::new(second),
            }),
        }
    }
}

impl Div for Op {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Op::Addition(divident), Op::Addition(divisor)) => divident / divisor,
            (Op::Multiplication(divident), Op::Multiplication(divisor)) => divident / divisor,
            (Op::Division(divident), Op::Division(divisor)) => divident / divisor,
            (Op::Number(divident), Op::Number(divisor)) => divident / divisor,
            (divident, divisor) => Op::Division(Division {
                divident: Box::new(divident),
                divisor: Box::new(divisor),
            }),
        }
    }
}

impl Mul for Op {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Op::Addition(first), Op::Addition(second)) => first * second,
            (Op::Multiplication(first), Op::Multiplication(second)) => first * second,
            (Op::Division(first), Op::Division(second)) => first * second,
            (Op::Number(first), Op::Number(second)) => first * second,

            (any, Op::Division(div)) => (any * (*div.divident)) / (*div.divisor),
            (Op::Division(div), any) => (any * (*div.divident)) / (*div.divisor),

            (first, second) => Op::Multiplication(Multiplication {
                multiplier: Box::new(first),
                multiplicand: Box::new(second),
            }),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Addition {
    pub first_summand: Box<Op>,
    pub second_summand: Box<Op>,
}

impl Calc for Addition {
    fn calc(&self) -> f64 {
        self.first_summand.calc() + self.second_summand.calc()
    }
}

impl Add for Addition {
    type Output = Op;

    fn add(self, rhs: Self) -> Self::Output {
        Op::Addition(Addition {
            first_summand: Box::new(Op::Addition(self)),
            second_summand: Box::new(Op::Addition(rhs)),
        })
    }
}

impl Mul for Addition {
    type Output = Op;

    fn mul(self, rhs: Self) -> Self::Output {
        Op::Multiplication(Multiplication {
            multiplier: Box::new(Op::Addition(self)),
            multiplicand: Box::new(Op::Addition(rhs)),
        })
    }
}

impl Div for Addition {
    type Output = Op;

    fn div(self, rhs: Self) -> Self::Output {
        Op::Division(Division {
            divident: Box::new(Op::Addition(self)),
            divisor: Box::new(Op::Addition(rhs)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Division {
    pub divident: Box<Op>,
    pub divisor: Box<Op>,
}

impl Calc for Division {
    fn calc(&self) -> f64 {
        self.divident.calc() / self.divisor.calc()
    }
}

impl Add for Division {
    type Output = Op;

    fn add(self, rhs: Self) -> Self::Output {
        if self.divisor == rhs.divisor {
            (*self.divident + *rhs.divident) / *self.divisor
        } else {
            let s_divident = *self.divident;
            let r_divident = *rhs.divident;
            let s_divisor = *self.divisor;
            let r_divisor = *rhs.divisor;

            ((s_divident.clone() * r_divisor.clone()) + (r_divident.clone() * s_divisor.clone()))
                / (s_divisor * r_divisor)
        }
    }
}

impl Mul for Division {
    type Output = Op;

    fn mul(self, rhs: Self) -> Self::Output {
        Op::Multiplication(Multiplication {
            multiplier: Box::new(Op::Division(self)),
            multiplicand: Box::new(Op::Division(rhs)),
        })
    }
}

impl Div for Division {
    type Output = Op;

    fn div(self, rhs: Self) -> Self::Output {
        Op::Division(Division {
            divident: Box::new(Op::Division(self)),
            divisor: Box::new(Op::Division(rhs)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Multiplication {
    multiplier: Box<Op>,
    multiplicand: Box<Op>,
}

impl Calc for Multiplication {
    fn calc(&self) -> f64 {
        self.multiplier.calc() * self.multiplicand.calc()
    }
}

impl Add for Multiplication {
    type Output = Op;

    fn add(self, rhs: Self) -> Self::Output {
        let s_multiplier = *self.multiplier;
        let s_multiplicand = *self.multiplicand;
        let r_multiplier = *rhs.multiplier;
        let r_multiplicand = *rhs.multiplicand;

        if s_multiplicand == r_multiplicand {
            s_multiplicand * (s_multiplier + r_multiplier)
        } else if s_multiplicand == r_multiplier {
            s_multiplicand * (s_multiplier + r_multiplicand)
        } else if s_multiplier == r_multiplicand {
            s_multiplier * (s_multiplicand + r_multiplier)
        } else if s_multiplier == r_multiplier {
            s_multiplier * (s_multiplicand + r_multiplicand)
        } else {
            Op::Addition(Addition {
                first_summand: Box::new(Op::Multiplication(Multiplication {
                    multiplier: Box::new(s_multiplier),
                    multiplicand: Box::new(s_multiplicand),
                })),
                second_summand: Box::new(Op::Multiplication(Multiplication {
                    multiplier: Box::new(r_multiplier),
                    multiplicand: Box::new(r_multiplicand),
                })),
            })
        }
    }
}

impl Mul for Multiplication {
    type Output = Op;

    fn mul(self, rhs: Self) -> Self::Output {
        Op::Multiplication(Multiplication {
            multiplier: Box::new(Op::Multiplication(self)),
            multiplicand: Box::new(Op::Multiplication(rhs)),
        })
    }
}

impl Div for Multiplication {
    type Output = Op;

    fn div(self, rhs: Self) -> Self::Output {
        if self.multiplier == rhs.multiplier {
            (*self.multiplicand) / (*rhs.multiplicand)
        } else if self.multiplier == rhs.multiplicand {
            (*self.multiplicand) / (*rhs.multiplier)
        } else if self.multiplicand == rhs.multiplier {
            (*self.multiplier) / (*rhs.multiplicand)
        } else if self.multiplicand == rhs.multiplicand {
            (*self.multiplier) / (*rhs.multiplier)
        } else {
            Op::Division(Division {
                divident: Box::new(Op::Multiplication(self)),
                divisor: Box::new(Op::Multiplication(rhs)),
            })
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
pub struct Integer {
    value: u32,
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer { value }
    }
}

impl Calc for Integer {
    fn calc(&self) -> f64 {
        f64::from(self.value)
    }
}

impl Add for Integer {
    type Output = Op;

    fn add(self, rhs: Self) -> Self::Output {
        Op::Number(Integer::from(self.value + rhs.value))
    }
}

impl Mul for Integer {
    type Output = Op;

    fn mul(self, rhs: Self) -> Self::Output {
        Op::Number(Integer::from(self.value * rhs.value))
    }
}

impl Div for Integer {
    type Output = Op;

    fn div(self, rhs: Self) -> Self::Output {
        if self.value % rhs.value == 0 {
            Op::Number(Integer::from(self.value / rhs.value))
        } else {
            let gcd = greatest_common_divisor(self.value, rhs.value);
            if gcd == 1 {
                Op::Division(Division {
                    divident: Box::new(Op::Number(self)),
                    divisor: Box::new(Op::Number(rhs)),
                })
            } else {
                Op::Division(Division {
                    divident: Box::new(Op::from(self.value / gcd)),
                    divisor: Box::new(Op::from(rhs.value / gcd)),
                })
            }
        }
    }
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    println!("Test implementation. Not for real world use.");

    let min = if a < b { a } else { b };

    for div in (2..=min).rev() {
        if a % div == 0 && b % div == 0 {
            return div;
        }
    }

    1
}
