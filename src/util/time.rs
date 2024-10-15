use chrono::{DateTime, Local, FixedOffset};

pub fn local_time_to_utc() -> DateTime<FixedOffset> {
    Local::now().with_timezone(&FixedOffset::east(8 * 3600))
}
