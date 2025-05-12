use sci_calc::{consts::PI, expr::*};

#[test]
fn eval() {
    assert_eq!(add(vec![num(1), num(1), num(1)]).simplify(), 3.0);
}

#[test]
fn simplify_easy() {
    assert_eq!(add(vec![num(1), num(1), neg(num(1))]).simplify(), num(1));
}

#[test]
fn simplify_hard() {
    let add = add(vec![num(1), num(1), num(1), neg(num(1))]);
    assert_eq!(add.simplify(), num(2));
}

// Here we start to see the benefit of this form of evaluation. Simply working with floating-point
// arithmetic would not produce the correct result here.
#[test]
fn simplify_const() {
    assert_eq!(add(vec![PI, num(1), neg(PI)]).simplify(), num(1));
}
