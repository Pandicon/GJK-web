use axum::response::IntoResponse;

pub const _ROUTE: &str = "/admin/new_user";
pub const _PERMISSIONS: &str = "ADMIN";
pub const _TYPE: &str = "POST";

#[derive(serde::Deserialize)]
pub struct User {
	pub mail : String,
	pub perms : u32
}

pub async fn callback(axum::extract::Json(user) : axum::extract::Json<User>) -> axum::response::Response<axum::body::Body> {
	let udb = crate::USER_DB.lock().unwrap();
	match udb.as_ref().unwrap().user_exists(&user.mail) {
		Ok(v) => if v { return (axum::http::StatusCode::CONFLICT, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("user {} already exists", user.mail) + "\"}").into_response() },
		Err(e) => {
			tracing::error!("Couldn't create user {} (perms {}): {}", user.mail, user.perms, e);
			return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("failed to check for user {} with permissions {}", user.mail, user.perms) + "\"}").into_response()
		}
	}
	match udb.as_ref().unwrap().add_user(&user.mail, user.perms) {
		Ok(_) => (axum::http::StatusCode::CREATED, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
			&format!("user {} with permissions {} created", user.mail, user.perms) + "\"}").into_response(),
		Err(e) => {
			tracing::error!("Couldn't create user {} (perms {}): {}", user.mail, user.perms, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"".to_owned() +
				&format!("failed to create user {} with permissions {}", user.mail, user.perms) + "\"}").into_response()
		}
	}
}
