use std::panic::resume_unwind;

pub fn raindrops(n: u32) -> String {
    let mut result: String = String::new();
    let is_factor = |factor| n % factor == 0;

    if is_factor(3) { result.push_str("Pling"); }
    if is_factor(5) { result.push_str("Plang"); }
    if is_factor(7) { result.push_str("Plong"); }

    if result.is_empty() { result = n.to_string() }

    result
}
