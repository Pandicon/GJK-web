use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub ip: String,
	pub port: String,
	pub supl_fetch_enabled: bool,
	pub debug_tokens_enabled: Option<bool>,
	pub calendar_cache_lifetime_sec: i64,
}

pub fn get_config() -> Config {
	let path = "./config.json";
	let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read the {:?} file.", path));
	let res: Config = serde_json::from_str(&data).expect("Unable to parse the configuration file.");
	res
}

#[derive(Serialize, Deserialize)]
pub struct GoogleCredentials {
	pub enabled: bool,
	pub api_key: String,
}

pub fn get_google_credentials_config() -> GoogleCredentials {
	let path = "./google_credentials.json";
	let data = std::fs::read_to_string(path);
	if data.is_err() {
		tracing::warn!("Google credentials file ({}) not found - disabling google calendar fetch.", path);
		return GoogleCredentials {
			enabled: false,
			api_key: String::new(),
		};
	}
	let mut res: GoogleCredentials = serde_json::from_str(&data.unwrap()).expect("Unable to parse the google credentials file.");
	res.enabled = true;
	res
}
