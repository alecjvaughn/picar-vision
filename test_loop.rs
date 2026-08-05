fn main() {
    let mut throttle = 0.5f32;
    let ly_d = 0.0f32;
    if throttle.abs() < ly_d { throttle = 0.0; } else { throttle = throttle.signum() * (throttle.abs() - ly_d) / (1.0 - ly_d.abs()); }
    println!("throttle: {}", throttle);
}
