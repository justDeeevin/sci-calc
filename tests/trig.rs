use sci_calc::cordic;

fn radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[test]
fn precision() {
    for theta in (-90..=90).step_by(15).map(f64::from) {
        let (cos, sin) = cordic(radians(theta));
        let cos_expected = radians(theta).cos();
        let sin_expected = radians(theta).sin();

        dbg!(theta, cos, cos_expected, sin, sin_expected);

        assert!((cos - cos_expected).abs() < 1e-12);
        assert!((sin - sin_expected).abs() < 1e-12);
    }
}
