use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OAuthConfig {
	#[serde(skip_deserializing)]
	pub enabled : bool,
	pub client_id : String,
	pub client_secret : String,
	pub redirect_uri : String,
}

pub fn get_oauth() -> OAuthConfig {
	let path = "./oauth.json";
	let data = std::fs::read_to_string(path);
	if data.is_err() {
		tracing::warn!("Google OAuth config file ({}) not found - disabling OAuth.", path);
		return OAuthConfig{ enabled: false, client_id: String::new(), client_secret: String::new(), redirect_uri: String::new() };
	}
	let mut res : OAuthConfig = serde_json::from_str(&unsafe {data.unwrap_unchecked()}).expect("Unable to parse the configuration file.");
	res.enabled = true;
	res
}
