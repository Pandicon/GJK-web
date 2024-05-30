use axum::response::IntoResponse;
use crate::article::Article;

pub const _ROUTE: &str = "/article/articles";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

#[derive(serde::Deserialize)]
pub struct Page {
	pub page : u32
}
#[derive(serde::Serialize)]
struct ArticleList {
	pub articles : Vec<Article>
}

pub async fn callback(axum::extract::Json(page): axum::extract::Json<Page>) -> axum::response::Response<axum::body::Body> {
	let adb = crate::ARTICLE_DB.lock().unwrap();
	match adb.as_ref().unwrap().get_chronol(page.page as usize, 10) {
		Ok(al) => {
			(
				axum::http::StatusCode::OK,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				match serde_json::ser::to_string(&ArticleList{articles: al}) {
					Ok(json) => json,
					Err(e) => {
						tracing::error!("Couldn't serialize articles: {}", e);
						return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to serialize articles\"}").into_response();
					}
				}
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't get articles: {}", e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to get articles\"}").into_response()
		}
	}
}
