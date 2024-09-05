use axum::response::IntoResponse;

pub const _ROUTE: &str = "/blob/new";
pub const _PERMISSIONS: &str = "MANAGE_ARTICLES";
pub const _TYPE: &str = "POST";

pub async fn callback(body : axum::body::Body) -> axum::response::Response<axum::body::Body> {
	let data = axum::body::to_bytes(body, usize::MAX).await.unwrap();
	let idb = crate::IMAGE_DB.lock().unwrap();
	match idb.as_ref().unwrap().add(&data[..]) {
		Ok(id) => {
			(
				axum::http::StatusCode::CREATED,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				"{\"message\":\"blob added\", \"id\":".to_owned() + &id.to_string() + "\"}",
			).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't add blob: {}", e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to create blob\"}").into_response()
		}
	}
}
