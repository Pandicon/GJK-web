use serde::{Deserialize, Serialize};

use super::event::CalendarEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct CalendarResponse {
	#[serde(rename = "accessRole")]
	access_role: String,
	#[serde(rename = "items")]
	pub events: Vec<CalendarEvent>,
}
