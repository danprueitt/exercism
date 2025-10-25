use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    const MINUTE: i32 = 1;
    const HOUR: i32 = 60 * Clock::MINUTE;
    const DAY: i32 = 24 * Clock::HOUR;

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: self::Clock::normalise_minutes(hours * Clock::HOUR + minutes) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }

    pub fn get_hour(&self) -> i32 {
        self.minutes / Clock::HOUR
    }

    pub fn get_minutes(&self) -> i32 {
        self.minutes % Clock::HOUR
    }

    fn normalise_minutes(minutes: i32) -> i32 {
        ((minutes % Clock::DAY) + Clock::DAY) % Clock::DAY
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.get_hour(), self.get_minutes())
    }
}
