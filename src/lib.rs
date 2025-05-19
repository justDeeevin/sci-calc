use std::sync::LazyLock;

// I do so wish these could be const...
pub const ITERATIONS: usize = 40;
static STEPS: LazyLock<[f64; ITERATIONS]> =
    LazyLock::new(|| std::array::from_fn(|i| 1_f64.atan2(2_f64.powi(i as i32))));
static K: LazyLock<f64> = LazyLock::new(|| {
    (0..(ITERATIONS as i32)).fold(1.0, |k, i| k * (1.0 / sqrt(1.0 + 2_f64.powi(-2 * i))))
});

pub fn cordic(beta: f64) -> (f64, f64) {
    let mut theta = 0.0;
    let mut point = (1.0, 0.0);
    let mut p2i = 1.0;

    for gamma in STEPS.iter() {
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

// TODO: manual implementation
pub fn sqrt(n: f64) -> f64 {
    n.sqrt()
}
