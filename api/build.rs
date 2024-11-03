use serde::Deserialize;

const ROUTES_FOLDER: &str = "./src/routes";

#[derive(Deserialize)]
struct PermissionFlagsInfo {
	permissions: u32,
	display_name: String,
	description: String,
}

fn get_permissions_flags() -> std::collections::HashMap<String, PermissionFlagsInfo> {
	let path = "./permission_flags.json";
	let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read the {:?} file.", path));
	let res: std::collections::HashMap<String, PermissionFlagsInfo> = serde_json::from_str(&data).expect("Unable to parse the permission flags file.");
	res
}

fn main() {
	let permissions_flags = get_permissions_flags();

	let out_dir = std::env::var("OUT_DIR").unwrap();
	let mut output = String::from("axum::Router::new()");
	let mut routes = vec![];

	fn read_folder(path: &std::path::Path, routes: &mut Vec<String>, output: &mut String, permissions_flags: &std::collections::HashMap<String, PermissionFlagsInfo>) {
		if path.is_dir() {
			if let Ok(entries) = std::fs::read_dir(path) {
				for entry in entries.flatten() {
					let path = entry.path();
					read_folder(&path, routes, output, permissions_flags);
				}
			}
		} else if path.is_file() && path.extension().unwrap_or_default() == "rs" && path.file_stem().unwrap_or_default() != "mod" {
			let contents = std::fs::read_to_string(path).unwrap();
			let mut route = String::from("");
			let mut req_type = String::from("");
			let mut permissions_vec: Vec<String> = vec![];
			for line in contents.lines() {
				if line.contains("ROUTE") && route.is_empty() {
					let value = line.split('=').nth(1).unwrap().trim().replace('"', "");
					if value.ends_with(';') {
						route = String::from(value.strip_suffix(';').unwrap());
					} else {
						route = value;
					}
				}
				if line.contains("TYPE") && req_type.is_empty() {
					let value = line.split('=').nth(1).unwrap().trim().to_lowercase().replace('"', "");
					if value.ends_with(';') {
						req_type = String::from(value.strip_suffix(';').unwrap());
					} else {
						req_type = value;
					}
				}
				if line.contains("PERMISSIONS") && permissions_vec.is_empty() {
					let value = line.split('=').nth(1).unwrap().trim().to_uppercase().replace('"', "");
					let val = if value.ends_with(';') { String::from(value.strip_suffix(';').unwrap()) } else { value };
					permissions_vec = val.split('|').map(|p| p.trim().to_owned()).collect();
				}
			}
			if route.is_empty() {
				panic!("ROUTE is empty in file {:?}", path.file_stem().unwrap_or_default());
			}
			if req_type.is_empty() || (req_type != "get" && req_type != "post" && req_type != "put" && req_type != "delete") {
				panic!("Invalid TYPE for route '{route}'. Expected 'GET', 'POST', 'PUT', or 'DELETE', got '{req_type}'");
			}
			let mut permissions: u32 = 0;
			for flag in permissions_vec {
				if let Some(flag_info) = permissions_flags.get(&flag) {
					if flag_info.permissions == 0 {
						continue;
					}
					permissions |= flag_info.permissions;
				} else {
					panic!("Invalid permission flag '{flag}' in route '{route}'");
				}
			}
			routes.push(route.clone());

			// Extremely readable :D
			// It takes the path, removes the file part ("./src/routes/supl/file.rs" -> "./src/routes/supl", removes the "./src" prefix -> "routes/supl", replaces all "\" with "/" (just to be sure) and replaces all "/" with "::" -> "routes::supl")
			let modules_prefix = String::from(path.parent().unwrap().strip_prefix("./src").unwrap().to_str().unwrap())
				.replace('\\', "/")
				.replace('/', "::");
			*output += format!(
				".route(\"{route}\", axum::routing::{req_type}({modules_prefix}::{}::callback).layer(axum::middleware::from_fn(user_data_middleware::attach_user_data)).layer(axum::middleware::from_fn_with_state({permissions}, permissions_middleware::check_permissions)))",
				path.file_stem().unwrap().to_str().unwrap()
			)
			.as_str();
		}
	}

	read_folder(std::path::Path::new(ROUTES_FOLDER), &mut routes, &mut output, &permissions_flags);

	std::fs::write(std::path::Path::new(&out_dir).join("router.rs"), format!("{{{output}}}")).unwrap();
	std::fs::write(std::path::Path::new(&out_dir).join("routes.rs"), format!("const GENERATED_ROUTES: &str = \"{}\";", routes.join(" | "))).unwrap();
	let mut codegen_map = phf_codegen::Map::new();
	let mut codegen_map = &mut codegen_map;
	for (flag, flag_info) in permissions_flags.iter() {
		codegen_map = codegen_map.entry(flag, &flag_info.permissions.to_string());
	}
	std::fs::write(
		std::path::Path::new(&out_dir).join("permission_flags.rs"),
		format!("pub static PERMISSION_FLAGS: phf::Map<&'static str, u32> = {};", codegen_map.build()),
	)
	.unwrap();
	std::fs::write(
		std::path::Path::new(&out_dir).join("permission_flags_info.rs"),
		format!(
			"lazy_static! {{pub static ref PERMISSION_FLAGS_INFO: Vec<crate::structs::permission_flags_info::PermissionFlagsInfo> = {{ let mut unsorted_vec = vec![{}]; unsorted_vec.sort_by_key(|a| a.get_flag().to_uppercase()); unsorted_vec }};}}",
			permissions_flags
				.iter()
				.map(|(flag, flag_info)| format!(
					"crate::structs::permission_flags_info::PermissionFlagsInfo::new(String::from(\"{}\"), {}, String::from(\"{}\"), String::from(\"{}\"))",
					flag, flag_info.permissions, flag_info.display_name, flag_info.description
				))
				.collect::<Vec<String>>()
				.join(", ")
		),
	)
	.unwrap();
}
