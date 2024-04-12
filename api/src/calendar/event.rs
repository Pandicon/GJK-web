use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
	#[serde(rename = "summary")]
	title: Option<String>,
	description: Option<String>,
	#[serde(rename = "htmlLink")]
	link: String,
	location: Option<String>,
	start: CalendarTime,
	end: CalendarTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarTime {
	#[serde(rename = "dateTime")]
	date_time: Option<String>,
	#[serde(rename = "timeZone")]
	timezone: Option<String>,
	date: Option<String>,
}
