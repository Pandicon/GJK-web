use axum::{response::IntoResponse, routing};

mod auth;
mod config;
mod permissions_middleware;
mod routes;
mod suplovani;

static SUPL: std::sync::Mutex<std::option::Option<suplovani::Suplovani>> = std::sync::Mutex::new(None);
pub static USER_DB: std::sync::Mutex<std::option::Option<auth::userdb::UserDB>> = std::sync::Mutex::new(None);
pub static TOKEN_STORAGE: std::sync::Mutex<std::option::Option<auth::token_storage::TokenStorage>> = std::sync::Mutex::new(None);

include!(concat!(std::env!("OUT_DIR"), "/permission_flags.rs"));
include!(concat!(std::env!("OUT_DIR"), "/routes.rs"));

#[tokio::main]
async fn main() {
	println!("{:?}", PERMISSION_FLAGS);
	dotenv::dotenv().ok();
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "INFO");
	}
	tracing_subscriber::fmt::init();

	let config = config::get_config();
	let oauth_config = auth::config::get_oauth();

	*USER_DB.lock().unwrap() = Some(auth::userdb::UserDB::new());
	*TOKEN_STORAGE.lock().unwrap() = Some(auth::token_storage::TokenStorage::new());

	let mut app = include!(concat!(std::env!("OUT_DIR"), "/router.rs"));

	if config.supl_fetch_enabled {
		suplovani::Suplovani::prepare();
		*SUPL.lock().unwrap() = Some(suplovani::Suplovani::new());
		SUPL.lock().unwrap().as_mut().unwrap().load();
		SUPL.lock().unwrap().as_mut().unwrap().start_thread(std::time::Duration::from_secs(900));
		app = app.route(
			"/supl",
			routing::get(|| async {
				let j = SUPL.lock().unwrap().as_ref().unwrap().get_json();
				([(axum::http::header::CONTENT_TYPE, "text/json")], j)
			}), //TODO: Specify permissions
				// .layer(axum::middleware::from_fn_with_state(17, permissions_middleware::check_permissions)),
		);
	}
	let mut _token_filter_thread = None; // for scoping, maybe this should be done through lifetimes?
	if oauth_config.enabled {
		app = app.route(
			"/auth/oauth",
			routing::get(|code: axum::extract::Query<auth::oauth::OAuthCode>| async move {
				match auth::oauth::get_email_from_code(&code.code, &oauth_config).await {
					Ok(mail) => {
						tracing::info!("user {} logged in", mail);
						let tokenstr = auth::token_storage::token_to_str(&TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().create(&mail));
						([(axum::http::header::CONTENT_TYPE, "text/json")], "{".to_owned()+&format!("\"token\":\"{}\"", tokenstr)+"}").into_response()
					},
					Err(e) => {
						tracing::error!("Error after OAuth callback - {:?}! (state = {})", e, code.state);
						(axum::http::StatusCode::BAD_REQUEST, "Code is missing").into_response()
					}
				}
			}),
		);
		_token_filter_thread = Some(std::thread::spawn(move || {
			loop {
				tracing::info!("filtering tokens...");
				TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().filter(3600 * 24 * 100);
				std::thread::sleep(std::time::Duration::from_secs(3600 * 12));
			}
		}));
	}

	let ip_and_port = config.ip + ":" + &config.port;
	let listener = tokio::net::TcpListener::bind(&ip_and_port).await.unwrap();
	tracing::info!("Listening on {}", ip_and_port);
	tracing::info!("Generated routes: {}", GENERATED_ROUTES);

	axum::serve(listener, app).await.unwrap();
}
