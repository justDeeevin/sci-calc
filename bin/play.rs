use sci_calc::{ITERATIONS, cordic};

fn radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

fn main() {
    dbg!(ITERATIONS);
    println!();
    println!("theta      sin(x)     diff. sine     cos(x)   diff. cosine ");
    for theta in (-90..=90).step_by(15).map(|i| i as f64) {
        let (cos, sin) = cordic(radians(theta));
        println!(
            "{theta:+05.1}°  {sin:+.8} ({:+.8}) {cos:+.8} ({:+.8})",
            sin - radians(theta).sin(),
            cos - radians(theta).cos()
        );
    }
}
