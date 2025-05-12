use sci_calc_macros::{constants, constructors};

use std::{
    collections::BTreeSet,
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub type Float = f64;

#[constructors]
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Expr {
    Num(Float),
    Neg(Box<Expr>),
    Add(Vec<Expr>),
    Constant(Const),
}

#[constants(Expr::Constant)]
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Const {
    Zero,
    Pi,
    E,
}

impl Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::Zero => write!(f, "0"),
            Const::Pi => write!(f, "π"),
            Const::E => write!(f, "e"),
        }
    }
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
            Expr::Constant(c) => write!(f, "{c}"),
        }
    }
}

impl Expr {
    /// Recursively evaluates the expression **without simplifying**.
    /// <div class="warning">
    /// Fox maximum precision, first simplify an expression then evaluate it.
    /// </div>
    pub fn eval(&self) -> Float {
        match self {
            Expr::Num(n) => *n,
            Expr::Neg(e) => -e.eval(),
            Expr::Add(set) => set.iter().fold(0.0, |acc, e| acc + e.eval()),
            Expr::Constant(c) => match c {
                Const::Zero => 0.0,
                Const::Pi => std::f64::consts::PI,
                Const::E => std::f64::consts::E,
            },
        }
    }

    /// Reduce the expression to its simplest form.
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
                    if let Some(test) = operands.iter().position(|test| test == &neg(e.clone())) {
                        remove_indices.insert(i);
                        remove_indices.insert(test);
                    }
                }

                for i in remove_indices.iter().rev() {
                    operands.remove(*i);
                }

                remove_indices.clear();

                if let Some(first_num) = operands.iter().position(|e| matches!(e, Expr::Num(_))) {
                    for i in (first_num + 1)..operands.len() {
                        if let Expr::Num(n) = operands[i] {
                            operands[first_num] += n;
                            remove_indices.insert(i);
                        }
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
            Expr::Constant(_) => self,
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

impl Add<Float> for Expr {
    type Output = Self;

    fn add(self, rhs: Float) -> Self::Output {
        self + num(rhs)
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

impl AddAssign<Float> for Expr {
    fn add_assign(&mut self, rhs: Float) {
        *self += num(rhs);
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

impl Sub<Float> for Expr {
    type Output = Self;

    fn sub(self, rhs: Float) -> Self::Output {
        self - num(rhs)
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

impl SubAssign<Float> for Expr {
    fn sub_assign(&mut self, rhs: Float) {
        *self -= num(rhs);
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

impl PartialEq<Float> for Expr {
    fn eq(&self, other: &Float) -> bool {
        self.eval().eq(other)
    }
}

impl PartialOrd<Float> for Expr {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        self.eval().partial_cmp(other)
    }
}

impl From<Expr> for Float {
    fn from(value: Expr) -> Self {
        value.simplify().eval()
    }
}

impl From<Float> for Expr {
    fn from(value: Float) -> Self {
        num(value)
    }
}
