use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Op {
    operation: Operation,
}

impl Op {
    pub fn calc(&self) -> f64 {
        self.operation.calc()
    }
    pub fn add(first: u32, second: u32) -> Self {
        Self::from(first + second)
    }
    pub fn mul(multiplier: u32, multiplicant: u32) -> Self {
        Self::from(multiplier * multiplicant)
    }
    pub fn div(divident: u32, divisor: u32) -> Self {
        Self::from(divident) / Self::from(divisor)
    }
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        Op {
            operation: Operation::from(value),
        }
    }
}

impl From<u16> for Op {
    fn from(value: u16) -> Self {
        Op::from(u32::from(value))
    }
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        Op::from(u32::from(value))
    }
}

impl From<i32> for Op {
    fn from(value: i32) -> Self {
        if value < 0 {
            Op {
                operation: -Operation::from(u32::try_from(value.abs()).unwrap()),
            }
        } else {
            Op {
                operation: Operation::from(u32::try_from(value).unwrap()),
            }
        }
    }
}

impl From<i16> for Op {
    fn from(value: i16) -> Self {
        Op::from(i32::from(value))
    }
}

impl From<i8> for Op {
    fn from(value: i8) -> Self {
        Op::from(i32::from(value))
    }
}

impl Default for Op {
    fn default() -> Self {
        Op {
            operation: Operation::default(),
        }
    }
}

impl Add for Op {
    type Output = Op;

    fn add(self, rhs: Self) -> Self::Output {
        Op {
            operation: self.operation + rhs.operation,
        }
    }
}

impl Sub for Op {
    type Output = Op;

    fn sub(self, rhs: Self) -> Self::Output {
        Op {
            operation: self.operation - rhs.operation,
        }
    }
}

impl Mul for Op {
    type Output = Op;

    fn mul(self, rhs: Self) -> Self::Output {
        Op {
            operation: self.operation * rhs.operation,
        }
    }
}

impl Div for Op {
    type Output = Op;

    fn div(self, rhs: Self) -> Self::Output {
        Op {
            operation: self.operation / rhs.operation,
        }
    }
}

impl Neg for Op {
    type Output = Op;

    fn neg(self) -> Self::Output {
        Op {
            operation: -self.operation,
        }
    }
}

trait Calc {
    fn calc(&self) -> f64;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Operation {
    Addition(Addition),
    Multiplication(Multiplication),
    Division(Division),
    Negation(Negation),
    Number(Number),
}

impl Calc for Operation {
    fn calc(&self) -> f64 {
        match self {
            Operation::Addition(add) => add.calc(),
            Operation::Multiplication(mul) => mul.calc(),
            Operation::Division(div) => div.calc(),
            Operation::Negation(inv) => inv.calc(),
            Operation::Number(num) => num.calc(),
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Number(Number::default())
    }
}

impl From<u32> for Operation {
    fn from(value: u32) -> Self {
        Operation::Number(Number { value })
    }
}

impl Add for Operation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first + second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first + second,
            (Operation::Division(first), Operation::Division(second)) => first + second,
            (Operation::Negation(first), Operation::Negation(second)) => first + second,
            (Operation::Number(first), Operation::Number(second)) => first + second,
            (first, second) => Operation::Addition(Addition {
                first_summand: Box::new(first),
                second_summand: Box::new(second),
            }),
        }
    }
}

impl Div for Operation {
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
            (divident, divisor) => Operation::Division(Division {
                divident: Box::new(divident),
                divisor: Box::new(divisor),
            }),
        }
    }
}

impl Mul for Operation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first * second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first * second,
            (Operation::Division(first), Operation::Division(second)) => first * second,
            (Operation::Negation(first), Operation::Negation(second)) => first * second,
            (Operation::Number(first), Operation::Number(second)) => first * second,

            (any, Operation::Division(div)) => (any * (*div.divident)) / (*div.divisor),
            (Operation::Division(div), any) => (any * (*div.divident)) / (*div.divisor),

            (first, second) => Operation::Multiplication(Multiplication {
                multiplier: Box::new(first),
                multiplicand: Box::new(second),
            }),
        }
    }
}

impl Sub for Operation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operation::Addition(first), Operation::Addition(second)) => first - second,
            (Operation::Multiplication(first), Operation::Multiplication(second)) => first - second,
            (Operation::Division(first), Operation::Division(second)) => first - second,
            (Operation::Negation(first), Operation::Negation(second)) => first - second,
            (Operation::Number(first), Operation::Number(second)) => first - second,
            (first, second) => Operation::Addition(Addition {
                first_summand: Box::new(first),
                second_summand: Box::new(Operation::Negation(Negation {
                    value: Box::new(second),
                })),
            }),
        }
    }
}

impl Neg for Operation {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        match self {
            Operation::Addition(add) => -add,
            Operation::Multiplication(mul) => -mul,
            Operation::Division(div) => -div,
            Operation::Negation(neg) => -neg,
            Operation::Number(num) => -num,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Negation {
    pub value: Box<Operation>,
}

impl Calc for Negation {
    fn calc(&self) -> f64 {
        -self.value.calc()
    }
}

impl Add for Negation {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new((*self.value) + (*rhs.value)),
        })
    }
}

impl Div for Negation {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        (*self.value) / (*rhs.value)
    }
}

impl Mul for Negation {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        (*self.value) * (*rhs.value)
    }
}

impl Sub for Negation {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        (*rhs.value) - (*self.value)
    }
}

impl Neg for Negation {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        *self.value
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Addition {
    pub first_summand: Box<Operation>,
    pub second_summand: Box<Operation>,
}

impl Calc for Addition {
    fn calc(&self) -> f64 {
        self.first_summand.calc() + self.second_summand.calc()
    }
}

impl Add for Addition {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Addition(Addition {
            first_summand: Box::new(Operation::Addition(self)),
            second_summand: Box::new(Operation::Addition(rhs)),
        })
    }
}

impl Mul for Addition {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multiplier: Box::new(Operation::Addition(self)),
            multiplicand: Box::new(Operation::Addition(rhs)),
        })
    }
}

impl Div for Addition {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Addition(self)),
            divisor: Box::new(Operation::Addition(rhs)),
        })
    }
}

impl Sub for Addition {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        Operation::Addition(self) + (-rhs)
    }
}

impl Neg for Addition {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Addition(Addition {
            first_summand: Box::new(-(*self.first_summand)),
            second_summand: Box::new(-(*self.second_summand)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Division {
    pub divident: Box<Operation>,
    pub divisor: Box<Operation>,
}

impl Calc for Division {
    fn calc(&self) -> f64 {
        self.divident.calc() / self.divisor.calc()
    }
}

impl Add for Division {
    type Output = Operation;

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

impl Mul for Division {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multiplier: Box::new(Operation::Division(self)),
            multiplicand: Box::new(Operation::Division(rhs)),
        })
    }
}

impl Div for Division {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(Division {
            divident: Box::new(Operation::Division(self)),
            divisor: Box::new(Operation::Division(rhs)),
        })
    }
}

impl Sub for Division {
    type Output = Operation;

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

impl Neg for Division {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Division(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct Multiplication {
    multiplier: Box<Operation>,
    multiplicand: Box<Operation>,
}

impl Calc for Multiplication {
    fn calc(&self) -> f64 {
        self.multiplier.calc() * self.multiplicand.calc()
    }
}

impl Add for Multiplication {
    type Output = Operation;

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
            Operation::Addition(Addition {
                first_summand: Box::new(Operation::Multiplication(Multiplication {
                    multiplier: Box::new(s_multiplier),
                    multiplicand: Box::new(s_multiplicand),
                })),
                second_summand: Box::new(Operation::Multiplication(Multiplication {
                    multiplier: Box::new(r_multiplier),
                    multiplicand: Box::new(r_multiplicand),
                })),
            })
        }
    }
}

impl Mul for Multiplication {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multiplier: Box::new(Operation::Multiplication(self)),
            multiplicand: Box::new(Operation::Multiplication(rhs)),
        })
    }
}

impl Div for Multiplication {
    type Output = Operation;

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
            Operation::Division(Division {
                divident: Box::new(Operation::Multiplication(self)),
                divisor: Box::new(Operation::Multiplication(rhs)),
            })
        }
    }
}

impl Sub for Multiplication {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        let s_multiplier = *self.multiplier;
        let s_multiplicand = *self.multiplicand;
        let r_multiplier = *rhs.multiplier;
        let r_multiplicand = *rhs.multiplicand;

        if s_multiplicand == r_multiplicand {
            s_multiplicand * (s_multiplier - r_multiplier)
        } else if s_multiplicand == r_multiplier {
            s_multiplicand * (s_multiplier - r_multiplicand)
        } else if s_multiplier == r_multiplicand {
            s_multiplier * (s_multiplicand - r_multiplier)
        } else if s_multiplier == r_multiplier {
            s_multiplier * (s_multiplicand - r_multiplicand)
        } else {
            Operation::Addition(Addition {
                first_summand: Box::new(Operation::Multiplication(Multiplication {
                    multiplier: Box::new(s_multiplier),
                    multiplicand: Box::new(s_multiplicand),
                })),
                second_summand: Box::new(-Operation::Multiplication(Multiplication {
                    multiplier: Box::new(r_multiplier),
                    multiplicand: Box::new(r_multiplicand),
                })),
            })
        }
    }
}

impl Neg for Multiplication {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Multiplication(self)),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
struct Number {
    value: u32,
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number { value }
    }
}

impl Calc for Number {
    fn calc(&self) -> f64 {
        f64::from(self.value)
    }
}

impl Add for Number {
    type Output = Operation;

    fn add(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value + rhs.value))
    }
}

impl Mul for Number {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Number(Number::from(self.value * rhs.value))
    }
}

impl Div for Number {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        if self.value % rhs.value == 0 {
            Operation::Number(Number::from(self.value / rhs.value))
        } else {
            let gcd = greatest_common_divisor(self.value, rhs.value);
            if gcd == 1 {
                Operation::Division(Division {
                    divident: Box::new(Operation::Number(self)),
                    divisor: Box::new(Operation::Number(rhs)),
                })
            } else {
                Operation::Division(Division {
                    divident: Box::new(Operation::from(self.value / gcd)),
                    divisor: Box::new(Operation::from(rhs.value / gcd)),
                })
            }
        }
    }
}

impl Sub for Number {
    type Output = Operation;

    fn sub(self, rhs: Self) -> Self::Output {
        Operation::from(self.value - rhs.value)
    }
}

impl Neg for Number {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Number(self)),
        })
    }
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    // euclidean algorithm

    let (mut smaller, mut bigger) = if a < b { (a, b) } else { (b, a) };

    while smaller != 0 {
        (smaller, bigger) = (bigger % smaller, smaller);
    }

    bigger
}
