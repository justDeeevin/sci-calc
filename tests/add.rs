use sci_calc::expr::*;

#[test]
fn simple_eval() {
    assert_eq!(add(vec![num(1.0), num(1.0), num(1.0)]).simplify(), 3.0);
}

#[test]
fn simplify_easy() {
    assert_eq!(
        add(vec![num(1.0), num(1.0), neg(num(1.0))]).simplify(),
        num(1.0)
    );
}
