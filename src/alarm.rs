use chrono::{NaiveTime, Weekday};

pub struct Alarm {
    pub id: u32,
    pub time: NaiveTime,
    pub repeat: Vec<Weekday>,
    pub soundfile: String,
    pub enabled: bool,
}
