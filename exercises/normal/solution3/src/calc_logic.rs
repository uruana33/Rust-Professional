pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0;
    }
    let mut t = 1.0;
    for i in 0..n {
        t *= (365 - i) as f64 / 365 as f64;
    }
    let s = format!("{:.4}", 1.0 - t);
    s.parse::<f64>().unwrap()
}
