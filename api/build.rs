const ROUTES_FOLDER: &str = "./src/routes";

fn main() {
	let out_dir = std::env::var("OUT_DIR").unwrap();
	let mut output = String::from("axum::Router::new()");
	let mut routes = vec![];

	fn read_folder(path: &std::path::Path, routes: &mut Vec<String>, output: &mut String) {
		if path.is_dir() {
			if let Ok(entries) = std::fs::read_dir(path) {
				for entry in entries {
					if let Ok(entry) = entry {
						let path = entry.path();
						read_folder(&path, routes, output);
					}
				}
			}
		} else if path.is_file() && path.extension().unwrap_or_default() == "rs" && path.file_stem().unwrap_or_default() != "mod" {
			let contents = std::fs::read_to_string(&path).unwrap();
			let mut route = String::from("");
			let mut req_type = String::from("");
			for line in contents.lines() {
				if line.contains("ROUTE") {
					let value = line.split('=').nth(1).unwrap().trim().replace("\"", "");
					if value.ends_with(";") {
						route = String::from(value.strip_suffix(";").unwrap());
					} else {
						route = value;
					}
				}
				if line.contains("TYPE") {
					let value = line.split('=').nth(1).unwrap().trim().to_lowercase().replace("\"", "");
					if value.ends_with(";") {
						req_type = String::from(value.strip_suffix(";").unwrap());
					} else {
						req_type = value;
					}
				}
			}
			if route.is_empty() || req_type.is_empty() || (req_type != "get" && req_type != "post") {
				return;
			}
			routes.push(route.clone());

			// Extremely readable :D
			// It takes the path, removes the file part ("./src/routes/supl/file.rs" -> "./src/routes/supl", removes the "./src" prefix -> "routes/supl", replaces all "\" with "/" (just to be sure) and replaces all "/" with "::" -> "routes::supl")
			let modules_prefix = String::from(path.parent().unwrap().strip_prefix("./src").unwrap().to_str().unwrap())
				.replace("\\", "/")
				.replace("/", "::");
			*output += format!(
				".route(\"{route}\", axum::routing::{req_type}({modules_prefix}::{}::callback))",
				path.file_stem().unwrap().to_str().unwrap()
			)
			.as_str();
		}
	}

	read_folder(std::path::Path::new(ROUTES_FOLDER), &mut routes, &mut output);

	std::fs::write(std::path::Path::new(&out_dir).join("router.rs"), format!("{{{output}}}")).unwrap();
	std::fs::write(
		std::path::Path::new(&out_dir).join("routes.rs"),
		format!("const GENERATED_ROUTES: &'static str = \"{}\";", routes.join(" | ")),
	)
	.unwrap();
}
