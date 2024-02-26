use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CalendarFetcher {
	credentials: CalendarCredentials,
	enabled: bool,
	cache_lifetime_sec: u32,
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

pub fn get_fetcher(cache_lifetime_sec_in: u32) -> CalendarFetcher {
	let path = "./skolni-web-calendar-credentials.json";
	let data = std::fs::read_to_string(path);
	if data.is_err() {
		tracing::warn!("Google Calendar config file ({}) not found - disabling Google Calendar.", path);
		return CalendarFetcher {
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
		};
	}
	let res: CalendarCredentials = serde_json::from_str(&data.unwrap()).expect("Unable to parse the calendar configuration file.");
	CalendarFetcher {
		credentials: res,
		enabled: true,
		cache_lifetime_sec: cache_lifetime_sec_in,
	}
}
