use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
	#[serde(rename = "summary")]
	title: Option<String>,
	description: Option<String>,
	#[serde(rename = "htmlLink")]
	link: String,
	location: Option<String>,
	pub start: CalendarTime,
	pub end: CalendarTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarTime {
	#[serde(rename = "dateTime")]
	pub date_time: Option<String>,
	#[serde(rename = "timeZone")]
	timezone: Option<String>,
	pub date: Option<String>,
	#[serde(skip_deserializing)]
	pub timestamp: i64,
}
