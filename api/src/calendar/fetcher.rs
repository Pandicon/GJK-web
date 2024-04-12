use std::collections::HashMap;

use crate::calendar::{cache::CalendarCache, event::CalendarEvent, response::CalendarResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CalendarFetcher {
	cache: CalendarCache,
	credentials: CalendarCredentials,
	enabled: bool,
	cache_lifetime_sec: i64,
	api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalendarCredentials {
	account_type: String,
	project_id: String,
	private_key_id: String,
	private_key: String,
	client_email: String,
	client_id: String,
	auth_uri: String,
	token_uri: String,
	auth_provider_x509_cert_url: String,
	client_x509_cert_url: String,
	universe_domain: String,

	calendar_id: String,
}

impl CalendarFetcher {
	pub async fn get_events(&mut self, start_date: Option<i64>, end_date: Option<i64>) -> Result<Option<Vec<CalendarEvent>>, reqwest::Error> {
		if !self.enabled {
			tracing::warn!("Google Calendar is not enabled");
			return Ok(None);
		}
		let start_time = start_date.unwrap_or(i64::MIN);
		let end_time = end_date.unwrap_or(i64::MAX);
		if self.cache.is_valid(self.cache_lifetime_sec) {
			tracing::debug!("Using the calendar cache");
			return Ok(Some(
				self.cache
					.events
					.clone()
					.into_iter()
					.filter(|event| event.start.timestamp >= start_time && event.end.timestamp <= end_time)
					.collect::<Vec<CalendarEvent>>(),
			));
		}
		tracing::debug!("Fetching Google Calendar events");
		let client = reqwest::Client::new();
		let mut params = HashMap::new();
		// TODO: Maybe only load events from the next year? Or even month? Right now it loads all events up to the end of 2043, including recurring events, which feels like a waste...
		// Could use something like this:
		/*if let Some(start_date) = start_date {
			if let Some(start_date) = crate::dates::unix_timestamp_to_rfc(start_date) {
				params.insert("timeMin", start_date);
			} else {
				tracing::error!("Invalid Unix timestamp: {start_date}");
			}
		}
		if let Some(end_date) = end_date {
			if let Some(end_date) = crate::dates::unix_timestamp_to_rfc(end_date) {
				params.insert("timeMax", end_date);
			} else {
				tracing::error!("Invalid Unix timestamp: {end_date}");
			}
		}*/
		params.insert("singleEvents", String::from("true"));
		params.insert("orderBy", String::from("startTime"));

		let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}", self.credentials.calendar_id, self.api_key);
		let response = client.get(&url).query(&params).send().await?.text().await?;

		let calendar_response_res = serde_json::from_str::<CalendarResponse>(&response);
		match calendar_response_res {
			Ok(mut calendar_response) => {
				self.cache.events = calendar_response
					.events
					.iter_mut()
					.map(|event| {
						if let Some(date) = &event.start.date {
							event.start.timestamp = crate::dates::rfc_to_unix_timestamp(&format!("{date}T00:00:00Z")).unwrap_or(0);
						}
						if let Some(date_time) = &event.start.date_time {
							event.start.timestamp = crate::dates::rfc_to_unix_timestamp(date_time).unwrap_or(0);
						}
						if let Some(date) = &event.end.date {
							event.end.timestamp = crate::dates::rfc_to_unix_timestamp(&format!("{date}T00:00:00Z")).unwrap_or(0);
						}
						if let Some(date_time) = &event.end.date_time {
							event.end.timestamp = crate::dates::rfc_to_unix_timestamp(date_time).unwrap_or(0);
						}
						event.clone()
					})
					.collect::<Vec<CalendarEvent>>();
				self.cache.refreshed();
			}
			Err(err) => {
				tracing::error!("Failed to deserialize the Google Calendar response: {:?}", err);
			}
		}
		Ok(Some(
			self.cache
				.events
				.clone()
				.into_iter()
				.filter(|event| event.start.timestamp >= start_time && event.end.timestamp <= end_time)
				.collect::<Vec<CalendarEvent>>(),
		))
	}
}

pub fn get_fetcher(cache_lifetime_sec_in: i64, api_key: String) -> CalendarFetcher {
	let path = "./skolni-web-calendar-credentials.json";
	let data = std::fs::read_to_string(path);
	if data.is_err() {
		tracing::warn!("Google Calendar config file ({}) not found - disabling Google Calendar.", path);
		return CalendarFetcher {
			cache: CalendarCache::new(),
			credentials: CalendarCredentials {
				account_type: String::new(),
				project_id: String::new(),
				private_key_id: String::new(),
				private_key: String::new(),
				client_email: String::new(),
				client_id: String::new(),
				auth_uri: String::new(),
				token_uri: String::new(),
				auth_provider_x509_cert_url: String::new(),
				client_x509_cert_url: String::new(),
				universe_domain: String::new(),

				calendar_id: String::new(),
			},
			enabled: false,
			cache_lifetime_sec: cache_lifetime_sec_in,
			api_key: String::new(),
		};
	}
	let res: CalendarCredentials = serde_json::from_str(&data.unwrap()).expect("Unable to parse the calendar configuration file.");
	CalendarFetcher {
		cache: CalendarCache::new(),
		credentials: res,
		enabled: true,
		cache_lifetime_sec: cache_lifetime_sec_in,
		api_key,
	}
}
