use axum::response::IntoResponse;
use crate::article::Article;

pub const _ROUTE: &str = "/article/new";
pub const _PERMISSIONS: &str = "MANAGE_ARTICLES";
pub const _TYPE: &str = "POST";

pub async fn callback(axum::extract::Json(u_article): axum::extract::Json<Article>) -> axum::response::Response<axum::body::Body> { // TODO: somehow control the author
	let adb = crate::ARTICLE_DB.lock().unwrap();
	let tm = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
	let mut article = u_article.clone();
	article.create_timestamp = tm;
	match adb.as_ref().unwrap().add(&article) {
		Ok(_) => {
			tracing::error!("Article {} by {} added", article.title, article.author);
			(
				axum::http::StatusCode::CREATED,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"".to_owned() + &format!("article {} by {} added", article.title, article.author) + "\"}",
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't add article: {}", e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to create article\"}").into_response()
		}
	}
}
