use axum::response::IntoResponse;

pub const _ROUTE: &str = "/auth/me";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

pub async fn callback(request : axum::extract::Request) -> axum::response::Response<axum::body::Body> {
	if let Some(token) = crate::auth::token_from_headers(&request) {
		if let Some(mail) = crate::TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().get(&token) {
			if let Ok(perms) = crate::USER_DB.lock().unwrap().as_ref().unwrap().get_perms(&mail) {
				([(axum::http::header::CONTENT_TYPE, "application/json")],
					format!("{{\"mail\":\"{}\",\"perms\":{}}}", mail, perms)).into_response()
			} else {
				(axum::http::StatusCode::INTERNAL_SERVER_ERROR,
					[(axum::http::header::CONTENT_TYPE, "application/json")],
					"{\"message\":\"couldn't find such user but token is valid\"}").into_response()
			}
		} else {
			(axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"token is invalid\"}").into_response()
		}
	} else {
		(axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"token not found in headers\"}").into_response()
	}
}

