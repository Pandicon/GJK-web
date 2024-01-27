use crate::auth;

#[derive(serde::Deserialize)]
pub struct OAuthCode {
	pub code: String,
	pub state: String,
}
#[derive(serde::Deserialize)]
pub struct OAuthTokens {
	pub access_token: String,
	pub id_token: String,
}
#[derive(serde::Deserialize)]
pub struct UserInfo {
	pub email: String,
}

pub async fn get_token(auth_code: &str, cfg: &auth::config::OAuthConfig) -> Result<OAuthTokens, Box<dyn std::error::Error>> {
	let root_url = "https://oauth2.googleapis.com/token";
	let client = reqwest::Client::new();
	let params = [
		("grant_type", "authorization_code"),
		("redirect_uri", &cfg.redirect_uri),
		("client_id", &cfg.client_id),
		("code", auth_code),
		("client_secret", &cfg.client_secret),
	];
	let resp = client.post(root_url).form(&params).send().await?;
	if resp.status().is_success() {
		Ok(serde_json::from_slice(&resp.bytes().await?)?)
	} else {
		let message = "Couldn't retrieve access token.";
		Err(From::from(message))
	}
}

pub async fn get_user_info(tokens: &OAuthTokens) -> Result<UserInfo, Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let mut url = reqwest::Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
	url.query_pairs_mut().append_pair("alt", "json");
	url.query_pairs_mut().append_pair("access_token", &tokens.access_token);
	let resp = client.get(url).bearer_auth(&tokens.id_token).send().await?;
	if resp.status().is_success() {
		Ok(serde_json::from_slice(&resp.bytes().await?)?)
	} else {
		let message = "Couldn't retrieve user information.";
		Err(From::from(message))
	}
}

pub async fn get_email_from_code(code: &str, cfg: &auth::config::OAuthConfig) -> Result<String, Box<dyn std::error::Error>> {
	let token = get_token(code, cfg).await?;
	let ui = get_user_info(&token).await?;
	Ok(ui.email)
}
