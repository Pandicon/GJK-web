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
	pub async fn get_events(&mut self, _start_date: Option<&str>, _end_date: Option<&str>) -> Result<Option<Vec<CalendarEvent>>, reqwest::Error> {
		if !self.enabled {
			tracing::warn!("Google Calendar is not enabled");
			return Ok(None);
		}
		if self.cache.is_valid(self.cache_lifetime_sec) {
			return Ok(Some(self.cache.events.clone()));
		}
		let client = reqwest::Client::new();
		let mut params = HashMap::new();
		/*if let Some(start_date) = start_date {
			params.insert("timeMin", start_date);
		}
		if let Some(end_date) = end_date {
			params.insert("timeMax", end_date);
		}*/
		// TODO: Maybe only load events from the next year? Or even month? Right now it loads all events up to the end of 2043, including recurring events, which feels like a waste...
		params.insert("singleEvents", "true");
		params.insert("orderBy", "startTime");

		let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}", self.credentials.calendar_id, self.api_key);
		let response = client.get(&url).query(&params).send().await?.text().await?;

		let calendar_response_res = serde_json::from_str::<CalendarResponse>(&response);
		match calendar_response_res {
			Ok(calendar_response) => {
				self.cache.events = calendar_response.events;
			}
			Err(err) => {
				tracing::error!("Failed to deserialize the Google Calendar response: {:?}", err);
			}
		}
		// TODO: Filter events by their start and end times
		/*let start_time = start_date.unwrap_or(i64::MIN);
		let end_time = end_date.unwrap_or(i64::MAX);*/
		Ok(Some(self.cache.events.clone()))
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
