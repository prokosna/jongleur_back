use chrono::prelude::*;

use domain::error::domain as ed;

pub fn convert_str_to_naive_date(date: &String) -> Result<NaiveDate, ed::Error> {
    if let Ok(parsed) = NaiveDate::parse_from_str(date, "%+") {
        Ok(parsed)
    } else {
        Err(
            ed::ErrorKind::InvalidRequest(format!("DateTime must be ISO8601/RFC3339: {}", date))
                .into(),
        )
    }
}
