const ITEMS_PER_HOUR: u8 = 221;
const ITEMS_PER_MINUTE: u8 = 221 / 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = get_success_rate(speed);

    ITEMS_PER_HOUR as f64 * success_rate * speed as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
 let success_rate = get_success_rate(speed);
 ITEMS_PER_MINUTE as u32 * success_rate as u32 * speed as u32
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