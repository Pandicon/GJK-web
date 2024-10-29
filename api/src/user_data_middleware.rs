use axum::{
	extract::Request,
	http::StatusCode,
	middleware::Next,
	response::Response,
};

pub async fn attach_user_data(mut request: Request, next: Next) -> Result<Response, StatusCode> {
	let mut user_data = None;
	if let Some(token) = crate::auth::token_from_headers(&request) {
		if let Some(mail) = crate::TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().get(&token) {
			match crate::USER_DB.lock().unwrap().as_ref().unwrap().get_user_opt(&mail) {
				Ok(Some(user)) => {
					user_data = Some(user);
				},
				Ok(None) => {
					user_data = None;
				},
				Err(err) => {
					tracing::error!("Error getting user data: {}", err);
					return Err(StatusCode::INTERNAL_SERVER_ERROR);
				}
			}
		}
	}
	request.extensions_mut().insert(user_data);
	let response = next.run(request).await;
	Ok(response)
}
