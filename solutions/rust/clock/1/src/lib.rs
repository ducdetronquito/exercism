use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: 24,
            minutes: 0,
        }
        .add_minutes(hours.rem_euclid(24) * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        let hours_to_add = total_minutes.div_euclid(60);
        let remaining_minutes = total_minutes.rem_euclid(60);
        Clock {
            hours: (self.hours + hours_to_add).rem_euclid(24),
            minutes: remaining_minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }
}
