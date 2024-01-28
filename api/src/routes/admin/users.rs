use axum::response::IntoResponse;

pub const _ROUTE: &str = "/admin/users";
pub const _PERMISSIONS: &str = "MANAGE_USERS";
pub const _TYPE: &str = "GET";

#[derive(serde::Serialize)]
struct User {
	pub mail: String,
	pub perms: u32,
}
#[derive(serde::Serialize)]
struct UserList {
	pub users: std::vec::Vec<User>,
}

pub async fn callback() -> axum::response::Response<axum::body::Body> {
	match crate::USER_DB.lock().unwrap().as_ref().unwrap().get() {
		Ok(ul) => {
			let mut l = UserList {
				users: std::vec::Vec::with_capacity(ul.len()),
			};
			for (m, p) in ul.iter() {
				l.users.push(User { mail: m.clone(), perms: *p });
			}
			axum::Json(l).into_response()
		}
		Err(e) => {
			tracing::error!("Couldn't get users: {}", e);
			(
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"couldn't get user list\"}",
			)
				.into_response()
		}
	}
}
