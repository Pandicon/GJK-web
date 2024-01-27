use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};

pub async fn check_permissions(State(required_permissions): State<u32>, request: Request, next: Next) -> Result<Response, StatusCode> {
	let user_permissions = 0; // TODO: Add proper permissions fetching from the database or other system

	if required_permissions & user_permissions != required_permissions {
		return Err(StatusCode::FORBIDDEN);
	}

	let response = next.run(request).await;
	Ok(response)
}
