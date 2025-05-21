#[test]
fn precision() {
    for n in (0..=100).map(f64::from) {
        let sqrt = sci_calc::sqrt(n);
        let sqrt_expected = n.sqrt();

        dbg!(n, sqrt, sqrt_expected);

        assert!((sqrt - sqrt_expected).abs() < 1e-14);
    }
}
