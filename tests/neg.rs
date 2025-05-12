use sci_calc::expr::*;

#[test]
fn eval() {
    assert_eq!(neg(num(1)), -1.0);
}

#[test]
fn simplify() {
    assert_eq!(neg(num(1)).simplify(), neg(num(1)));
}

#[test]
fn double_neg() {
    assert_eq!(neg(neg(num(1))).simplify(), num(1));
}

#[test]
fn quad_neg() {
    assert_eq!(neg(neg(neg(neg(num(1))))).simplify(), num(1));
}
