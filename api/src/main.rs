use axum::{response::IntoResponse, routing};
use lazy_static::lazy_static;

mod auth;
mod config;
mod permissions_middleware;
mod routes;
mod structs;
mod suplovani;

static SUPL: std::sync::Mutex<std::option::Option<suplovani::Suplovani>> = std::sync::Mutex::new(None);
pub static USER_DB: std::sync::Mutex<std::option::Option<auth::userdb::UserDB>> = std::sync::Mutex::new(None);
pub static TOKEN_STORAGE: std::sync::Mutex<std::option::Option<auth::token_storage::TokenStorage>> = std::sync::Mutex::new(None);

include!(concat!(std::env!("OUT_DIR"), "/permission_flags.rs"));
include!(concat!(std::env!("OUT_DIR"), "/permission_flags_info.rs"));
include!(concat!(std::env!("OUT_DIR"), "/routes.rs"));

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "INFO");
	}
	tracing_subscriber::fmt::init();

	let config = config::get_config();
	let oauth_config = auth::config::get_oauth();

	*USER_DB.lock().unwrap() = Some(auth::userdb::UserDB::new());
	*TOKEN_STORAGE.lock().unwrap() = Some(auth::token_storage::TokenStorage::new());
	USER_DB.lock().unwrap().as_ref().unwrap().print().unwrap();

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
				([(axum::http::header::CONTENT_TYPE, "application/json")], j)
			})
			.layer(axum::middleware::from_fn_with_state(
				*PERMISSION_FLAGS.get("READ_SUBSTITUTIONS").unwrap(),
				permissions_middleware::check_permissions,
			)),
		);
	}
	let mut _token_filter_thread = None; // for scoping, maybe this should be done through lifetimes?
	if oauth_config.enabled {
		app = app.route(
			"/auth/oauth",
			routing::get(|code: axum::extract::Query<auth::oauth::OAuthCode>| async move {
				match auth::oauth::get_email_from_code(&code.code, &oauth_config).await {
					Ok(mail) => {
						if mail.ends_with("@gjk.cz") {
							let perms = USER_DB
								.lock()
								.unwrap()
								.as_ref()
								.unwrap()
								.get_perms_or_add_with(&mail, *crate::PERMISSION_FLAGS.get("GJK_DEFAULT").unwrap());
							match perms {
								Ok(p) => {
									tracing::info!("gjk user {} logged in with perms {}", mail, p);
								}
								Err(e) => {
									tracing::error!("gjk user {} logged in, but the server couldn't get perms: {}", mail, e);
									return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Couldn't find or create user.").into_response();
								}
							}
						} else {
							let perms = USER_DB.lock().unwrap().as_ref().unwrap().get_perms_opt(&mail);
							match perms {
								Ok(po) => match po {
									Some(p) => tracing::info!("non-gjk user {} logged in with perms {}", mail, p),
									None => {
										tracing::info!("non-gjk user {} can't log in without pre-existing user", mail);
										return (axum::http::StatusCode::FORBIDDEN, format!("e-mail {} isn't registered", mail)).into_response();
									}
								},
								Err(e) => {
									tracing::error!("non-gjk user {} logged in, but the server couldn't get perms: {}", mail, e);
									return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Couldn't find user.").into_response();
								}
							}
						}
						let ts = TOKEN_STORAGE.lock().unwrap();
						if let Some(tokens) = ts.as_ref().unwrap().iget(&mail) {
							if tokens.len() > 20 {
								tracing::warn!("User {} has too many tokens ({}), removing (-> replacing) oldest token", mail, tokens.len());
								ts.as_ref().unwrap().remove(tokens.iter().min_by_key(|t| auth::token_storage::token_timestamp(t)).unwrap());
							}
						}
						let tokenstr = auth::token_storage::token_to_str(&ts.as_ref().unwrap().create(&mail));
						([(axum::http::header::CONTENT_TYPE, "application/json")], "{".to_owned() + &format!("\"token\":\"{}\"", tokenstr) + "}").into_response()
					}
					Err(e) => {
						tracing::error!("Error after OAuth callback - {:?}! (state = {})", e, code.state);
						(axum::http::StatusCode::BAD_REQUEST, "Error occured during OAuth.").into_response()
					}
				}
			}),
		);
		_token_filter_thread = Some(std::thread::spawn(move || loop {
			tracing::info!("filtering tokens...");
			TOKEN_STORAGE.lock().unwrap().as_ref().unwrap().filter(3600 * 24 * 100);
			std::thread::sleep(std::time::Duration::from_secs(3600 * 12));
		}));
	}

	let cors = tower_http::cors::CorsLayer::new()
		.allow_methods([http::Method::GET, http::Method::POST])
		.allow_origin(tower_http::cors::Any);
	app = app.layer(cors);

	let ip_and_port = config.ip + ":" + &config.port;
	let listener = tokio::net::TcpListener::bind(&ip_and_port).await.unwrap();
	tracing::info!("Listening on {}", ip_and_port);
	tracing::info!("Generated routes: {}", GENERATED_ROUTES);
	tracing::info!("Permission flags: {:#?}", PERMISSION_FLAGS);
	tracing::info!(
		"Permission flags info: {:#?}",
		PERMISSION_FLAGS_INFO.iter().collect::<Vec<&crate::structs::permission_flags_info::PermissionFlagsInfo>>()
	); // This is due to the PERMISSION_FLAGS_INFO variable being initialised via lazy_static!, so the type is a bit weird I suppose. But it should behave just as a normal Vec<crate::structs::permission_flags_info::PermissionFlagsInfo> in other cases.

	axum::serve(listener, app).await.unwrap();
}
