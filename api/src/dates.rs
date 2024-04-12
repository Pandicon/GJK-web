use chrono::{DateTime, NaiveDateTime, Utc};

pub fn unix_timestamp() -> i64 {
	Utc::now().timestamp()
}

#[allow(dead_code)] // It may be useful
pub fn unix_timestamp_to_rfc(unix_timestamp: i64) -> Option<String> {
	match NaiveDateTime::from_timestamp_opt(unix_timestamp, 0) {
		Some(datetime) => {
			let date = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
			let date_str = date.to_rfc3339();
			Some(date_str)
		}
		None => None,
	}
}

pub fn rfc_to_unix_timestamp(rfc: &str) -> Option<i64> {
	match rfc.parse::<DateTime<Utc>>() {
		Ok(date) => {
			let timestamp = date.timestamp();
			Some(timestamp)
		}
		Err(err) => {
			tracing::error!("Invalid rfc date: {:?}\nErr: {:?}", rfc, err);
			None
		}
	}
}
