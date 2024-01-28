#[derive(Debug)]
pub struct PermissionFlagsInfo {
	flag: String,
	permissions: u32,
	display_name: String,
	description: String,
}

impl PermissionFlagsInfo {
	pub fn new(flag: String, permissions: u32, display_name: String, description: String) -> Self {
		Self {
			flag,
			permissions,
			display_name,
			description,
		}
	}

	pub fn get_description(&self) -> &str {
		&self.description
	}

	pub fn get_display_name(&self) -> &str {
		&self.display_name
	}

	pub fn get_flag(&self) -> &str {
		&self.flag
	}

	pub fn get_permissions(&self) -> u32 {
		self.permissions
	}
}