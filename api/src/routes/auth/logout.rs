use axum::response::IntoResponse;

pub const _ROUTE: &str = "/auth/logout";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "POST";

pub async fn callback(request : axum::extract::Request) -> axum::response::Response<axum::body::Body> {
	if let Some(token) = crate::auth::token_from_headers(&request) {
		let ts = crate::TOKEN_STORAGE.lock().unwrap();
		if ts.as_ref().unwrap().remove(&token) {
			([(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"logged out successfuly\"}").into_response()
		} else {
			(axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"token is invalid\"}").into_response()
		}
	} else {
		(axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"token not found in headers\"}").into_response()
	}
}
