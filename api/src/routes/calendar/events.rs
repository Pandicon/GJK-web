use crate::CALENDAR_FETCHER;
use axum::response::IntoResponse;

pub const _ROUTE: &str = "/calendar/events";
pub const _PERMISSIONS: &str = "READ_CALENDAR";
pub const _TYPE: &str = "GET";

#[derive(serde::Deserialize)]
pub struct GetCalendarEventsQueryParams {
	start_time: Option<i64>,
	end_time: Option<i64>,
}

#[derive(serde::Serialize)]
struct EventsList {
	pub events: std::vec::Vec<crate::calendar::event::CalendarEvent>,
	pub cached_at: i64,
}

pub async fn callback(axum::extract::Query(params): axum::extract::Query<GetCalendarEventsQueryParams>) -> axum::response::Response<axum::body::Body> {
	let cf_clone = std::sync::Arc::clone(&CALENDAR_FETCHER);
	let res = tokio::spawn(async move {
		let mut calendar_fetcher = cf_clone.lock().await;
		calendar_fetcher.as_mut().unwrap().get_events(params.start_time, params.end_time).await
	})
	.await
	.unwrap();
	match res {
		Ok(Some(events)) => {
			tracing::info!("Loaded calendar events: {:#?}", events);
			let events_response = EventsList {
				events,
				cached_at: CALENDAR_FETCHER.lock().await.as_ref().unwrap().cache_refreshed_at(),
			};
			axum::Json(events_response).into_response()
		}
		Ok(None) => (
			axum::http::StatusCode::SERVICE_UNAVAILABLE,
			[(axum::http::header::CONTENT_TYPE, "application/json")],
			"{\"message\":\"Google Calendar is not enabled on the backend\"}".to_owned(),
		)
			.into_response(),
		Err(error) => {
			tracing::error!("Error when loading calendar events: {:#?}", error);
			(
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				[(axum::http::header::CONTENT_TYPE, "application/json")],
				format!("{{\"message\":\"An error occured when loading calendar events: {:#?}\"}}", error),
			)
				.into_response()
		}
	}
}
