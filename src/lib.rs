use sci_calc_macros::{constants, constructors};

use std::{
    collections::BTreeSet,
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub type Float = f64;

#[derive(Debug, PartialEq, Clone)]
#[constructors]
#[non_exhaustive]
pub enum Expr {
    Num(Float),
    Neg(Box<Expr>),
    Add(Vec<Expr>),
    Constant(Const),
}

#[derive(Debug, PartialEq, Clone)]
#[constants(Expr::Constant)]
pub enum Const {
    Zero,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Neg(e) => write!(f, "-({e})"),
            Expr::Add(operands) => write!(
                f,
                "{}",
                operands
                    .iter()
                    .fold(String::new(), |acc, e| format!("{acc} + ({e})"))
            ),
            Expr::Constant(_) => todo!(),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Float {
        match self {
            Expr::Num(n) => *n,
            Expr::Neg(e) => -e.eval(),
            Expr::Add(set) => set.iter().fold(0.0, |acc, e| acc + e.eval()),
            Expr::Constant(_) => todo!(),
        }
    }

    pub fn simplify(self) -> Expr {
        match self {
            Expr::Num(_) => self,
            Expr::Neg(e) => match *e {
                Expr::Neg(e) => e.simplify(),
                e => neg(e),
            },
            Expr::Add(operands) => {
                let mut operands = operands
                    .into_iter()
                    .map(|e| e.simplify())
                    .collect::<Vec<_>>();
                let mut remove_indices = BTreeSet::new();
                for (i, e) in operands.iter().enumerate() {
                    if let Some(test) = operands
                        .iter()
                        .position(|test| test == &neg(e.clone()) && !remove_indices.contains(&i))
                    {
                        remove_indices.insert(i);
                        remove_indices.insert(test);
                    }
                }

                for i in remove_indices.iter().rev() {
                    operands.remove(*i);
                }

                if operands.len() == 1 {
                    operands.pop().unwrap()
                } else if operands.is_empty() {
                    ZERO
                } else {
                    add(operands)
                }
            }
            Expr::Constant(_) => todo!(),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if let Self::Add(mut operands) = self {
            operands.push(rhs);
            add(operands)
        } else if let Self::Add(mut operands) = rhs {
            operands.push(self);
            add(operands)
        } else if let (Self::Num(left), Self::Num(right)) = (&self, &rhs) {
            num(left + right)
        } else {
            add(vec![self, rhs])
        }
    }
}

impl AddAssign for Expr {
    fn add_assign(&mut self, rhs: Self) {
        if let Self::Add(operands) = self {
            operands.push(rhs);
        } else if let (Self::Num(left), Self::Num(right)) = (&self, &rhs) {
            *self = num(*left + *right);
        } else {
            *self = add(vec![self.clone(), rhs])
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Self::Add(mut operands) = self {
            operands.push(neg(rhs));
            add(operands)
        } else if let Self::Add(mut operands) = rhs {
            operands.push(neg(self));
            add(operands)
        } else if let (Self::Num(left), Self::Num(right)) = (&self, &rhs) {
            num(left - right)
        } else {
            add(vec![self, neg(rhs)])
        }
    }
}

impl SubAssign for Expr {
    fn sub_assign(&mut self, rhs: Self) {
        if let Self::Add(operands) = self {
            operands.push(neg(rhs));
        } else if let (Self::Num(left), Self::Num(right)) = (&self, &rhs) {
            *self = num(*left - *right);
        } else {
            *self = add(vec![self.clone(), neg(rhs)])
        }
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Self::Neg(e) = self {
            *e
        } else {
            neg(self)
        }
    }
}
