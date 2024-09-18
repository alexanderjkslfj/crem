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

const MAX_U32_AS_F64: f64 = u32::MAX as f64;

fn f64_to_u32(value: f64) -> Result<u32, ()> {
    if value.fract() != 0.0 {
        return Err(());
    }
    if value > MAX_U32_AS_F64 {
        return Err(());
    }
    Ok(value as u32)
}

impl TryFrom<f64> for Op {
    type Error = ();

    fn try_from(mut value: f64) -> Result<Self, Self::Error> {
        if value == 0.0 {
            return Ok(Op::from(0u32));
        }

        let invert = value.is_sign_negative();
        if invert {
            value = -value;
        }

        let mut exponent = 0;

        loop {
            let offset = 10.0f64.powi(exponent);

            let raised = value * offset;

            if raised == raised.trunc() {
                if invert {
                    return Ok(-Op::div(f64_to_u32(raised)?, offset as u32));
                } else {
                    return Ok(Op::div(f64_to_u32(raised)?, offset as u32));
                }
            }

            exponent += 1;
        }
    }
}

const MAX_U32_AS_F32: f32 = u32::MAX as f32;

fn f32_to_u32(value: f32) -> Result<u32, ()> {
    if value.fract() != 0.0 {
        return Err(());
    }
    if value > MAX_U32_AS_F32 {
        return Err(());
    }
    Ok(value as u32)
}

impl TryFrom<f32> for Op {
    type Error = ();

    fn try_from(mut value: f32) -> Result<Self, Self::Error> {
        if value == 0.0 {
            return Ok(Op::from(0u32));
        }

        let invert = value.is_sign_negative();
        if invert {
            value = -value;
        }

        let mut exponent = 0;

        loop {
            let offset = 10.0f32.powi(exponent);

            let raised = value * offset;

            if raised == raised.trunc() {
                if invert {
                    return Ok(-Op::div(f32_to_u32(raised)?, offset as u32));
                } else {
                    return Ok(Op::div(f32_to_u32(raised)?, offset as u32));
                }
            }

            exponent += 1;
        }
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
                operation: -Operation::from(value.abs() as u32),
            }
        } else {
            Op {
                operation: Operation::from(value.abs() as u32),
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

            (Operation::Number(num), any) if (num.value == 0) => any,
            (any, Operation::Number(num)) if (num.value == 0) => any,

            // experimental
            (Operation::Division(div), any) => {
                (any * (*div.divisor).clone() + (*div.divident)) / (*div.divisor)
            }
            (any, Operation::Division(div)) => {
                (any * (*div.divisor).clone() + (*div.divident)) / (*div.divisor)
            }

            (Operation::Negation(neg), any) => any - (*neg.value),
            (any, Operation::Negation(neg)) => any - (*neg.value),

            (first, second) => Operation::Addition(Addition {
                summands: vec![first, second],
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

            (_, Operation::Number(num)) if (num.value == 0) => panic!("Cannot divide by zero."),
            (any, Operation::Number(num)) if (num.value == 1) => any,
            (Operation::Number(num), _) if (num.value == 0) => Operation::Number(num),

            (Operation::Negation(neg), any) => -((*neg.value) / any),
            (any, Operation::Negation(neg)) => -(any / (*neg.value)),

            (any, Operation::Division(div)) => any * ((*div.divisor) / (*div.divident)),
            (Operation::Division(div), any) => {
                Operation::from(1) / (any * ((*div.divisor) / (*div.divident)))
            }

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

            (Operation::Number(num), _) if (num.value == 0) => Operation::Number(num),
            (_, Operation::Number(num)) if (num.value == 0) => Operation::Number(num),
            (Operation::Number(num), any) if (num.value == 1) => any,
            (any, Operation::Number(num)) if (num.value == 1) => any,

            (any, Operation::Negation(neg)) => -(any * (*neg.value)),
            (Operation::Negation(neg), any) => -((*neg.value) * any),

            (any, Operation::Division(div)) => (any * (*div.divident)) / (*div.divisor),
            (Operation::Division(div), any) => (any * (*div.divident)) / (*div.divisor),

            (first, second) => Operation::Multiplication(Multiplication {
                multipliers: vec![first, second],
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

            (Operation::Number(num), any) if (num.value == 0) => -any,
            (any, Operation::Number(num)) if (num.value == 0) => any,

            (Operation::Negation(neg), any) => -((*neg.value) + any),
            (any, Operation::Negation(neg)) => any + (*neg.value),

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
    pub summands: Vec<Operation>,
}

impl Calc for Addition {
    fn calc(&self) -> f64 {
        self.summands.iter().map(|summand| summand.calc()).sum()
    }
}

impl Add for Addition {
    type Output = Operation;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.summands.append(&mut rhs.summands);
        Operation::Addition(Addition {
            summands: self.summands,
        })
    }
}

impl Mul for Addition {
    type Output = Operation;

    fn mul(self, rhs: Self) -> Self::Output {
        Operation::Multiplication(Multiplication {
            multipliers: vec![Operation::Addition(self), Operation::Addition(rhs)],
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

impl Neg for Addition {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        Operation::Negation(Negation {
            value: Box::new(Operation::Addition(self)),
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
        ((*self.divident) * (*rhs.divident)) / ((*self.divisor) * (*rhs.divisor))
    }
}

impl Div for Division {
    type Output = Operation;

    fn div(self, rhs: Self) -> Self::Output {
        Operation::Division(self) * ((*rhs.divisor) / (*rhs.divident))
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
    multipliers: Vec<Operation>,
}

impl Calc for Multiplication {
    fn calc(&self) -> f64 {
        self.multipliers
            .iter()
            .map(|multiplier| multiplier.calc())
            .product()
    }
}

impl Add for Multiplication {
    type Output = Operation;

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

impl Mul for Multiplication {
    type Output = Operation;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        // TODO: optimize
        self.multipliers.append(&mut rhs.multipliers);
        Operation::Multiplication(Multiplication {
            multipliers: self.multipliers,
        })
    }
}

impl Div for Multiplication {
    type Output = Operation;

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

impl Sub for Multiplication {
    type Output = Operation;

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
        if self.value < rhs.value {
            -Operation::from(rhs.value - self.value)
        } else {
            Operation::from(self.value - rhs.value)
        }
    }
}

impl Neg for Number {
    type Output = Operation;

    fn neg(self) -> Self::Output {
        if self.value == 0 {
            Operation::Number(self)
        } else {
            Operation::Negation(Negation {
                value: Box::new(Operation::Number(self)),
            })
        }
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
