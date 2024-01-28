use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};

pub async fn check_permissions(State(required_permissions): State<u32>, request: Request, next: Next) -> Result<Response, StatusCode> {
	if required_permissions == 0 { // early exit to avoid potential unnecessary userdb/tokenstorage locks
		return Ok(next.run(request).await);
	}
	let mut user_permissions = 0;
	if let Some(token) = crate::auth::token_from_headers(&request) {
		if let Some(mail) = crate::TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().get(&token) {
			if let Ok(perms) = crate::USER_DB.lock().unwrap().as_ref().unwrap().get_perms(&mail) {
				user_permissions = perms;
			}
		}
	}

	if required_permissions & user_permissions != required_permissions {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let response = next.run(request).await;
	Ok(response)
}
