use axum::response::IntoResponse;
use axum::extract::Extension;
use crate::auth::userdb::User;

pub const _ROUTE: &str = "/article/delete";
pub const _PERMISSIONS: &str = "MANAGE_ARTICLES";
pub const _TYPE: &str = "DELETE";

#[derive(serde::Deserialize)]
pub struct ArticleID {
	pub id : i64
}

pub async fn callback(Extension(user_data): Extension<Option<User>>, axum::extract::Query(a_id): axum::extract::Query<ArticleID>) -> axum::response::Response<axum::body::Body> {
	let id = a_id.id;
	let user_data = match user_data {
		Some(ud) => ud,
		None => {
			tracing::error!("User data not attached to a 'delete article' endpoint call.");
			return (
				http::StatusCode::INTERNAL_SERVER_ERROR,
				[(http::header::CONTENT_TYPE, "application/json")],
				"{\"message\": \"Failed to get user data\"}"
			).into_response();
		}
	};
	let adb = crate::ARTICLE_DB.lock().unwrap();
	let article_data = match adb.as_ref().unwrap().get(id) {
		Ok(Some(article)) => article,
		Ok(None) => {
			tracing::debug!("Article with id {} does not exist", id);
			return (
				axum::http::StatusCode::NOT_FOUND,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("The article with id {} does not exist", id) + "\"}",
			).into_response();
		},
		Err(err) => {
			tracing::error!("Failed to fetch the article data: {}", err);
			return (
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"Unable to delete the article\"}"
			).into_response();
		}
	};
	if article_data.author_email != user_data.mail {
		return (
			axum::http::StatusCode::FORBIDDEN,
			[(axum::http::header::CONTENT_TYPE, "application/json")],
			"{\"message\":\"Only the author can delete an article.\"}",
		).into_response()
	}
	match adb.as_ref().unwrap().delete(id) {
		Ok(_) => {
			tracing::info!("Article {} with id {} by {} deleted", article_data.title, article_data.id, user_data.mail);
			(
				axum::http::StatusCode::NO_CONTENT,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't delete article with id {}: {}", article_data.id, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"Unable to delete article\"}").into_response()
		}
	}
}
