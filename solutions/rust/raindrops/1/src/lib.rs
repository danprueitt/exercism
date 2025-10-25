use std::panic::resume_unwind;

pub fn raindrops(n: u32) -> String {
    let mut result: String = String::from("");
    if n % 3 == 0 {
        result += "Pling"
    }
    if n % 5 == 0
    {
        result += "Plang"
    }
    if n % 7 == 0 {
        result += "Plong"
    }

    if n%3 != 0 && n%5 != 0 && n%7 != 0 {
        result = n.to_string()
    }

    result
}
