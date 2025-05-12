use sci_calc_macros::{constants, constructors};

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub type Float = f64;
pub type Int = u128;

#[constructors]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub enum Expr {
    Num(Int),
    Neg(Box<Expr>),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Frac(Box<Expr>, Box<Expr>),
    Constant(Const),
}

#[constants(Expr::Constant)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
                    .fold(String::new(), |acc, e| if acc.is_empty() {
                        format!("({e})")
                    } else {
                        format!("{acc} + ({e})")
                    })
            ),
            Expr::Constant(c) => write!(f, "{c}"),
            Expr::Mul(operands) => write!(
                f,
                "{}",
                operands
                    .iter()
                    .fold(String::new(), |acc, e| if acc.is_empty() {
                        format!("({e})")
                    } else {
                        format!("{acc} * ({e})")
                    })
            ),
            Expr::Frac(a, b) => write!(f, "({a})/({b})"),
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
            Expr::Num(n) => *n as Float,
            Expr::Neg(e) => -e.eval(),
            Expr::Add(set) => set.iter().fold(0.0, |acc, e| acc + e.eval()),
            Expr::Mul(set) => set.iter().fold(1.0, |acc, e| acc * e.eval()),
            Expr::Constant(c) => match c {
                Const::Zero => 0.0,
                Const::Pi => std::f64::consts::PI,
                Const::E => std::f64::consts::E,
            },
            Expr::Frac(a, b) => a.eval() / b.eval(),
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
                let operands = operands
                    .into_iter()
                    .map(|e| e.simplify())
                    .collect::<Vec<_>>();
                let mut operands_map = HashMap::new();
                for e in operands {
                    operands_map.entry(e).and_modify(|c| *c += 1).or_insert(1);
                }

                let mut canceled = HashMap::new();

                for (expr, count) in &operands_map {
                    if let Some(count_neg) = operands_map.get(&neg(expr.clone())) {
                        canceled.insert(expr.clone(), *count_neg.min(count));
                    }
                }

                for (expr, count) in canceled {
                    *operands_map.get_mut(&expr).unwrap() -= count;
                    *operands_map.get_mut(&neg(expr)).unwrap() -= count;
                }

                operands_map.retain(|_, count| *count > 0);

                let mut operands = operands_map
                    .into_iter()
                    .flat_map(|(e, c)| vec![e; c as usize])
                    .collect::<Vec<_>>();

                let mut removed_indices = Vec::new();
                if let Some(first_num) = operands.iter().position(|e| matches!(e, Expr::Num(_))) {
                    for i in (first_num + 1)..operands.len() {
                        if let Expr::Num(n) = operands[i] {
                            operands[first_num] += n;
                            removed_indices.push(i);
                        }
                    }
                }

                for i in removed_indices.iter().rev() {
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
            Expr::Mul(_) => todo!(),
            Expr::Frac(..) => todo!(),
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

impl Add<Int> for Expr {
    type Output = Self;

    fn add(self, rhs: Int) -> Self::Output {
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

impl AddAssign<Int> for Expr {
    fn add_assign(&mut self, rhs: Int) {
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

impl Sub<Int> for Expr {
    type Output = Self;

    fn sub(self, rhs: Int) -> Self::Output {
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

impl SubAssign<Int> for Expr {
    fn sub_assign(&mut self, rhs: Int) {
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

impl From<Int> for Expr {
    fn from(value: Int) -> Self {
        num(value)
    }
}
