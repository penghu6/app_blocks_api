use chrono::{DateTime, Local, TimeZone, Utc};

pub fn local_time_to_utc() -> DateTime<Utc> {
    let local_time = Local::now().naive_local();
    Local.from_local_datetime(&local_time).unwrap().with_timezone(&Utc)
}
