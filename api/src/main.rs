use axum::routing;

mod config;
mod suplovani;

static SUPL : std::sync::Mutex<std::option::Option<suplovani::Suplovani>> = std::sync::Mutex::new(None);

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "INFO");
	}
	tracing_subscriber::fmt::init();

	let config = config::get_config();

	let mut app = axum::Router::new().route("/", routing::get(|| async { "Hi" }));

	suplovani::Suplovani::prepare();
	*SUPL.lock().unwrap() = Some(suplovani::Suplovani::new());
	SUPL.lock().unwrap().as_mut().unwrap().load();
	SUPL.lock().unwrap().as_mut().unwrap().start_thread(std::time::Duration::from_secs(900));
	app = app.route("/supl", routing::get(|| async {
		let j = SUPL.lock().unwrap().as_ref().unwrap().get_json();
		([(axum::http::header::CONTENT_TYPE, "text/json")], j)
	}));

	let ip_and_port = config.ip + ":" + &config.port;
	let listener = tokio::net::TcpListener::bind(&ip_and_port).await.unwrap();
	tracing::info!("Listening on {}", ip_and_port);

	axum::serve(listener, app).await.unwrap();
}
