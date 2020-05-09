use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: u32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * 60 + minutes).rem_euclid(1440) as u32
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock{
            minutes: (self.minutes as i32 + minutes).rem_euclid(1440) as u32
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.minutes / 60, self.minutes % 60)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes / 60, self.minutes % 60)
    }
}

impl From::<Clock> for String {
    fn from(clock: Clock) -> Self {
        format!("{:0>2}:{:0>2}", clock.minutes / 60, clock.minutes % 60)
    }
}
