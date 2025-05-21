
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n*factorial(n-1)
    }
}
