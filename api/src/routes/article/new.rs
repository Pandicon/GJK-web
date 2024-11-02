use axum::response::IntoResponse;
use axum::extract::Extension;
use crate::article::Article;
use crate::auth::userdb::User;

pub const _ROUTE: &str = "/article/new";
pub const _PERMISSIONS: &str = "MANAGE_ARTICLES";
pub const _TYPE: &str = "POST";

pub async fn callback(Extension(user_data): Extension<Option<User>>, axum::extract::Json(u_article): axum::extract::Json<Article>) -> axum::response::Response<axum::body::Body> { // TODO: somehow control the author
	let user_data = match user_data {
		Some(ud) => ud,
		None => {
			tracing::error!("User data not attached to a 'create article' endpoint call.");
			return (
				http::StatusCode::INTERNAL_SERVER_ERROR,
				[(http::header::CONTENT_TYPE, "application/json")],
				"{\"message\": \"Failed to get user data\"}"
			).into_response();
		}
	};
	let adb = crate::ARTICLE_DB.lock().unwrap();
	let tm = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
	let mut article = u_article.clone();
	article.create_timestamp = tm;
	article.author_email = user_data.mail;
	article.author_name = user_data.name;
	match adb.as_ref().unwrap().add(&article) {
		Ok(_) => {
			tracing::info!("Article {} by {} added", article.title, article.author_email);
			(
				axum::http::StatusCode::CREATED,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("article {} by {} added", article.title, article.author_email) + "\"}",
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't add article: {}", e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to create article\"}").into_response()
		}
	}
}
