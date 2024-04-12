use crate::calendar::event::CalendarEvent;

#[derive(Debug)]
pub struct CalendarCache {
	pub events: Vec<CalendarEvent>,
	created_at: i64,
}

impl CalendarCache {
	pub fn new() -> Self {
		Self { events: Vec::new(), created_at: 0 }
	}

	pub fn is_valid(&self, cache_validity_s: i64) -> bool {
		self.created_at + cache_validity_s > chrono::Utc::now().timestamp()
	}
}
