use axum::response::IntoResponse;

pub const _ROUTE: &str = "/blob/get";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

#[derive(serde::Deserialize)]
pub struct IdType {
	pub id : i64
}

pub async fn callback(axum::extract::Query(id): axum::extract::Query<IdType>) -> axum::response::Response<axum::body::Body> {
	let idb = crate::IMAGE_DB.lock().unwrap();
	match idb.as_ref().unwrap().get(id.id) {
		Ok(data) => {
			let content_type : &'static str;
			if data.len() < 4 {
				tracing::error!("Blob is too short to have magic");
				return (axum::http::StatusCode::OK, data).into_response()
			} else if data[0..4] == [0xFF, 0xD8, 0xFF, 0xE0] || data[0..4] == [0xFF, 0xD8, 0xFF, 0xEE] {
				content_type = "image/jpg";
			} else if data.len() < 8 {
				tracing::warn!("Blob is short, maybe unknown magic: {:02x} {:02x} {:02x} {:02x}", data[0], data[1], data[2], data[3]);
				return (axum::http::StatusCode::OK, data).into_response()
			} else if data[0..8] == [0x25, 0x50, 0x44, 0x46, 0x2D] {
				content_type = "application/pdf";
			} else if data[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
				content_type = "image/png";
			} else {
				tracing::error!("Unknown blob magic: {:02x} {:02x} {:02x} {:02x}", data[0], data[1], data[2], data[3]);
				return (axum::http::StatusCode::OK, data).into_response()
			}
			(axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, content_type)], data).into_response()
		},
		Err(e) => {
			tracing::error!("Couldn't get blob {}: {}", id.id, e);
			(axum::http::StatusCode::INTERNAL_SERVER_ERROR, [(axum::http::header::CONTENT_TYPE, "application/json")], "{\"message\":\"unable to get articles\"}").into_response()
		}
	}
}
