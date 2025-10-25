const ITEMS_PER_HOUR: f64 = 221.0;
const ITEMS_PER_MINUTE: f64 = 221.0 / 60.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = get_success_rate(speed);

    ITEMS_PER_HOUR * success_rate * (speed as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
 let success_rate = get_success_rate(speed);
    (ITEMS_PER_MINUTE * success_rate * (speed as f64)) as u32
}

fn get_success_rate(speed: u8) -> f64 {
    if speed < 5 {
        1.0
    } else if speed < 9 {
        0.9
    } else {
        0.77
    }
}