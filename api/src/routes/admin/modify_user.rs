use axum::response::IntoResponse;
use crate::auth::userdb::User;

pub const _ROUTE: &str = "/admin/modify_user";
pub const _PERMISSIONS: &str = "MANAGE_USERS";
pub const _TYPE: &str = "POST";

pub async fn callback(axum::extract::Json(user): axum::extract::Json<User>) -> axum::response::Response<axum::body::Body> {
	let udb = crate::USER_DB.lock().unwrap();
	match udb.as_ref().unwrap().user_exists(&user.mail) {
		Ok(v) => {
			if !v {
				return (
					axum::http::StatusCode::CONFLICT,
					[(axum::http::header::CONTENT_TYPE, "application/json")],
					"{\"message\":\"".to_owned() + &format!("user {} doesn't exist", user.mail) + "\"}",
				)
					.into_response();
			}
		}
		Err(e) => {
			tracing::error!("Couldn't check for user {}: {}", user.mail, e);
			return (
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("failed to update user {} with permissions {}", user.mail, user.perms) + "\"}",
			)
				.into_response();
		}
	}
	match udb.as_ref().unwrap().update_perms(&user.mail, user.perms) {
		Ok(_) => {},
		Err(e) => {
			tracing::error!("Couldn't update perms for user {} (new perms {}): {}", user.mail, user.perms, e);
			return (
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("failed to update user {} with permissions {}", user.mail, user.perms) + "\"}",
			)
				.into_response();
		}
	}
	match udb.as_ref().unwrap().update_name(&user.mail, user.name.as_deref()) {
		Ok(_) => {},
		Err(e) => {
			tracing::error!("Couldn't update name for user {} (new name {:?}): {}", user.mail, user.name, e);
			return (
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("failed to update user {} with name {}", user.mail, user.name.unwrap_or_else(|| String::from("None"))) + "\"}",
			)
				.into_response();
		}
	}
	(axum::http::StatusCode::CREATED, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"updated\"}").into_response()
}
