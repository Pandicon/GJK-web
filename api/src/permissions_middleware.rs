use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};

pub async fn check_permissions(State(required_permissions): State<u32>, request: Request, next: Next) -> Result<Response, StatusCode> {
	let mut user_permissions = 0;
	if let Some(auth_header) = request.headers().get("authorization") {
		if let Ok(auth_header_str) = auth_header.to_str() {
			if auth_header_str.starts_with("Bearer ") || auth_header_str.starts_with("bearer ") {
				if let Ok(token) = crate::auth::token_storage::token_from_str(&auth_header_str[7..]) {
					if let Some(mail) = crate::TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().get(&token) {
						if let Ok(perms) = crate::USER_DB.lock().unwrap().as_ref().unwrap().get_perms(&mail) {
							user_permissions = perms;
						}
					}
				}
			}
		}
	}

	if required_permissions & user_permissions != required_permissions {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let response = next.run(request).await;
	Ok(response)
}
