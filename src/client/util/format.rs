use std::time;

pub fn time_to_local(systime: time::SystemTime, date_format: &str) -> String {
    let datetime: chrono::DateTime<chrono::offset::Local> = systime.into();
    datetime.format(date_format).to_string()
}

pub fn time_to_utc(systime: time::SystemTime, date_format: &str) -> String {
    let datetime: chrono::DateTime<chrono::offset::Utc> = systime.into();
    datetime.format(date_format).to_string()
}
