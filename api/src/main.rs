use axum::routing;

mod config;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "INFO");
	}
	tracing_subscriber::fmt::init();

	let config = config::get_config();

	let app = axum::Router::new().route("/", routing::get(|| async { "Hi" }));
	let ip_and_port = config.ip + ":" + &config.port;
	let listener = tokio::net::TcpListener::bind(&ip_and_port).await.unwrap();
	tracing::info!("Listening on {}", ip_and_port);

	axum::serve(listener, app).await.unwrap();
}
