use std::ops::RangeFrom;

pub fn nth(n: u32) -> u32 {
    (RangeFrom { start: 2 })
        .filter(|x| is_prime(x))
        .nth(n as usize)
        .unwrap()
}

pub fn is_prime(n: &u32) -> bool {
    !(2..*n).any(|x| n % x == 0)
}
