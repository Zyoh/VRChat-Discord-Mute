use chrono::prelude::{DateTime, Utc};

pub fn iso8601() -> String {
    let time = std::time::SystemTime::now();
    let dt: DateTime<Utc> = time.into();
    format!("{}", dt.format("%+"))
}
