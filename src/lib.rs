use std::sync::LazyLock;

// I do so wish these could be const...
pub const CORDIC_ITERATIONS: usize = 40;
static CORDIC_STEP_ANGLES: LazyLock<[f64; CORDIC_ITERATIONS]> =
    LazyLock::new(|| std::array::from_fn(|i| 1_f64.atan2(2_f64.powi(i as i32))));
static K: LazyLock<f64> = LazyLock::new(|| {
    (0..(CORDIC_ITERATIONS as i32)).fold(1.0, |k, i| k * (1.0 / sqrt(1.0 + 2_f64.powi(-2 * i))))
});

pub fn cordic(beta: f64) -> (f64, f64) {
    let mut theta = 0.0;
    let mut point = (1.0, 0.0);
    let mut p2i = 1.0;

    for gamma in CORDIC_STEP_ANGLES.iter() {
        let sigma = if theta < beta { 1.0 } else { -1.0 };
        theta += sigma * gamma;
        point = (
            point.0 - sigma * point.1 * p2i,
            point.1 + sigma * p2i * point.0,
        );
        p2i /= 2.0;
    }

    (point.0 * *K, point.1 * *K)
}

pub fn sin(theta: f64) -> f64 {
    cordic(theta).1
}

pub fn cos(theta: f64) -> f64 {
    cordic(theta).0
}

pub const SQRT_ITERATIONS: usize = 10;

// TODO: manual implementation
pub fn sqrt(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }
    if n < 0.0 {
        unimplemented!("Complex math");
    }
    (0..SQRT_ITERATIONS).fold(initial_estimate(n), |x, _| 0.5 * (x + (n / x)))
}

pub fn initial_estimate(n: f64) -> f64 {
    const EXP_MASK: u64 = 0x7FF0000000000000;
    const MANTISSA_MASK: u64 = 0x000FFFFFFFFFFFFF;

    let bits = n.to_bits();
    let exp = ((bits & EXP_MASK) >> 52) as i32;
    let mant = bits & MANTISSA_MASK;

    let new_exp = ((exp - 1023) / 2 + 1023) as u64;
    let estmate_bits = (new_exp << 52) | mant;

    f64::from_bits(estmate_bits)
}
