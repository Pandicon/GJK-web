pub mod config;
pub mod oauth;
pub mod userdb;
pub mod token_storage;

pub fn token_from_headers(request: &axum::extract::Request) -> Option<token_storage::Token> {
	if let Some(auth_header) = request.headers().get("authorization") {
		if let Ok(auth_header_str) = auth_header.to_str() {
			if auth_header_str.starts_with("Bearer ") || auth_header_str.starts_with("bearer ") {
				if let Ok(token) = token_storage::token_from_str(&auth_header_str[7..]) {
					return Some(token);
				}
			}
		}
	}
	None
}
