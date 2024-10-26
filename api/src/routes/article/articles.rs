use axum::response::IntoResponse;
use crate::article::ArticleWithAuthorName;
use crate::USER_DB;

pub const _ROUTE: &str = "/article/articles";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

#[derive(serde::Deserialize)]
pub struct Page {
	pub page : u32
}
#[derive(serde::Serialize)]
struct ArticleList {
	pub articles : Vec<ArticleWithAuthorName>
}

pub async fn callback(axum::extract::Query(page): axum::extract::Query<Page>) -> axum::response::Response<axum::body::Body> {
	let adb = crate::ARTICLE_DB.lock().unwrap();
	match adb.as_ref().unwrap().get_chronol(page.page as usize, 10) {
		Ok(al) => {
			let articles_with_author_names = al.into_iter().map(|article| {
				let author_name = match USER_DB.lock().unwrap().as_ref().unwrap().get_name(&article.author_email) {
					Ok(name) => name,
					Err(err) => {
						tracing::error!("Failed to get name of user {}: {}", article.author_email, err);
						None
					}
				};
				ArticleWithAuthorName::from_article(article, author_name)
			}).collect();
			(
				axum::http::StatusCode::OK,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				match serde_json::ser::to_string(&ArticleList{articles: articles_with_author_names}) {
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
