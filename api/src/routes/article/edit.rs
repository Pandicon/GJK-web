use axum::response::IntoResponse;
use axum::extract::Extension;
use crate::article::Article;
use crate::auth::userdb::User;

pub const _ROUTE: &str = "/article/edit";
pub const _PERMISSIONS: &str = "MANAGE_ARTICLES";
pub const _TYPE: &str = "POST";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditArticle {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub tags: Vec<String>,
	pub thumbnail_id: u64,
}

pub async fn callback(Extension(user_data): Extension<Option<User>>, axum::extract::Json(u_article): axum::extract::Json<EditArticle>) -> axum::response::Response<axum::body::Body> {
	let user_data = match user_data {
		Some(ud) => ud,
		None => {
			tracing::error!("User data not attached to an 'edit article' endpoint call.");
			return (
				http::StatusCode::INTERNAL_SERVER_ERROR,
				[(http::header::CONTENT_TYPE, "application/json")],
				"{\"message\": \"Failed to get user data\"}"
			).into_response();
		}
	};
	let adb = crate::ARTICLE_DB.lock().unwrap();
	let old_article_data = match adb.as_ref().unwrap().get(u_article.id) {
		Ok(Some(article)) => article,
		Ok(None) => {
			tracing::debug!("Article with id {} does not exist", u_article.id);
			return (
				axum::http::StatusCode::NOT_FOUND,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("The article with id {} does not exist", u_article.id) + "\"}",
			).into_response();
		},
		Err(err) => {
			tracing::error!("Failed to fetch the old article data: {}", err);
			return (
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"Unable to edit the article\"}"
			).into_response();
		}
	};
	if old_article_data.author_email != user_data.mail {
		return (
			axum::http::StatusCode::FORBIDDEN,
			[(axum::http::header::CONTENT_TYPE, "application/json")],
			"{\"message\":\"Only the author can edit an article.\"}",
		).into_response()
	}
	match adb.as_ref().unwrap().edit(u_article.id, &u_article) {
		Ok(_) => {
			tracing::info!("Article {} with id {} by {} edited", u_article.title, u_article.id, user_data.mail);
			(
				axum::http::StatusCode::OK,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("Article {} with id {} by {} edited", u_article.title, u_article.id, user_data.mail) + "\"}",
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't edit article with id {}: {}", u_article.id, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"Unable to edit article\"}").into_response()
		}
	}
}
