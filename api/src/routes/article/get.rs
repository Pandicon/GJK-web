use axum::response::IntoResponse;
use crate::article::ArticleWithoutAuthorEmail;

pub const _ROUTE: &str = "/article/get";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

#[derive(serde::Deserialize)]
pub struct ArticleID {
	pub id : u32
}
pub async fn callback(axum::extract::Query(aid): axum::extract::Query<ArticleID>) -> axum::response::Response<axum::body::Body> {
	let adb = crate::ARTICLE_DB.lock().unwrap();
	match adb.as_ref().unwrap().get(aid.id as i64) {
		Ok(Some(a)) => {
			let a = ArticleWithoutAuthorEmail::from_article(a);
			(
				axum::http::StatusCode::OK,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				match serde_json::ser::to_string(&a) {
					Ok(json) => json,
					Err(e) => {
						tracing::error!("Couldn't serialize the article: {}", e);
						return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"Unable to serialize the article\"}").into_response();
					}
				}
			).into_response()
		},
		Ok(None) => {
			tracing::debug!("Article with id {} does not exist", aid.id);
			(axum::http::StatusCode::NOT_FOUND, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"Article does not exist\"}").into_response()
		}
		Err(e) => {
			// TODO: return other error if the article doesn't exist
			tracing::error!("Couldn't get the article (id {}): {}", aid.id, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"Unable to get the article\"}").into_response()
		}
	}
}
