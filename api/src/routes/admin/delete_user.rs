use axum::response::IntoResponse;

pub const _ROUTE: &str = "/admin/delete_user";
pub const _PERMISSIONS: &str = "ADMIN";
pub const _TYPE: &str = "POST";

#[derive(serde::Deserialize)]
pub struct User {
	pub mail : String
}

pub async fn callback(axum::extract::Json(user) : axum::extract::Json<User>) -> axum::response::Response<axum::body::Body> {
	let udb = crate::USER_DB.lock().unwrap();
	match udb.as_ref().unwrap().user_exists(&user.mail) {
		Ok(v) => if !v { return (axum::http::StatusCode::CONFLICT, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("user {} doesn't exist", user.mail) + "\"}").into_response() },
		Err(e) => {
			tracing::error!("Couldn't check for user {}: {}", user.mail, e);
			return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("failed to delete user {}", user.mail) + "\"}").into_response()
		}
	}
	match udb.as_ref().unwrap().remove_user(&user.mail) {
		Ok(_) => (axum::http::StatusCode::CREATED, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"user removed\"}").into_response(),
		Err(e) => {
			tracing::error!("Couldn't delete user {}: {}", user.mail, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("failed to delete user {}", user.mail) + "\"}").into_response()
		}
	}
}
