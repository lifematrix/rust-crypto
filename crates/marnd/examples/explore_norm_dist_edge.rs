use core::f64;

fn main() {
    let x = f64::MIN_POSITIVE;
    let y = x.ln();
    println!("MIN_POSITIVE {}, {}", x, y);

    let x = f64::EPSILON;
    let y = x.ln();
    println!("EPSILON {}, {}", x, y);
}
