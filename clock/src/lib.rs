use std::fmt::Display;

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * MINUTES_IN_HOUR + minutes;
        let normalized_minutes_in_day = total_minutes.rem_euclid(HOURS_IN_DAY * MINUTES_IN_HOUR);

        let hours = (normalized_minutes_in_day / MINUTES_IN_HOUR) % HOURS_IN_DAY;
        let minutes = normalized_minutes_in_day % MINUTES_IN_HOUR;

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
