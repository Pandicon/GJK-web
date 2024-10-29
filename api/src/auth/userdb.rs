const USERDB_FILE: &str = "./userdb.db";
pub struct UserDB {
	con: rusqlite::Connection,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
	pub mail : String,
	pub name: Option<String>,
	pub perms : u32
}

impl UserDB {
	pub fn new() -> Self {
		let first = !std::path::Path::new(USERDB_FILE).exists();
		let con = rusqlite::Connection::open(USERDB_FILE).unwrap();
		let out = Self { con };
		if first {
			out.create_table();
		}
		out
	}

	pub fn add_user(&self, mail: &str, name: Option<&str>, perms: u32) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("INSERT INTO user VALUES (?1, ?2, ?3);", rusqlite::params![mail, name, perms])?;
		Ok(())
	}
	pub fn remove_user(&self, mail: &str) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("DELETE FROM user WHERE mail = ?1;", [mail])?;
		Ok(())
	}
	pub fn update_perms(&self, mail: &str, perms: u32) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("UPDATE user SET perms = ?1 WHERE mail = ?2;", rusqlite::params![perms, mail])?;
		Ok(())
	}
	pub fn update_name(&self, mail: &str, name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("UPDATE user SET name = ?1 WHERE mail = ?2;", rusqlite::params![name, mail])?;
		crate::ARTICLE_DB.lock().unwrap().as_ref().unwrap().rename_author(mail, name)?; // TODO: What if this call fails?
		Ok(())
	}
	pub fn get_perms(&self, mail: &str) -> Result<u32, Box<dyn std::error::Error>> {
		match self.get_perms_opt(mail) {
			Ok(Some(perms)) => Ok(perms),
			Ok(None) => Err(format!("User with mail {} doesn't exist.", mail).into()),
			Err(e) => Err(e)
		}
	}
	pub fn get_perms_opt(&self, mail: &str) -> Result<Option<u32>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT perms FROM user WHERE mail = ?1;")?;
		if let Ok(perms) = s.query_row([mail], |r| { let x : u32 = r.get(0)?; Ok(x) } ) {
			return Ok(Some(perms));
		}
		Ok(None)
	}
	pub fn get_name(&self, mail: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
		match self.get_name_opt(mail) {
			Ok(Some(name)) => Ok(name),
			Ok(None) => Err(format!("User with mail {} doesn't exist.", mail).into()),
			Err(e) => Err(e)
		}
	}
	pub fn get_name_opt(&self, mail: &str) -> Result<Option<Option<String>>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT name FROM user WHERE mail = ?1;")?;
		if let Ok(name) = s.query_row([mail], |r| { let x: Option<String> = r.get(0)?; Ok(x) } ) {
			return Ok(Some(name));
		}
		Ok(None)
	}
	pub fn get_user(&self, mail: &str) -> Result<User, Box<dyn std::error::Error>> {
		match self.get_user_opt(mail) {
			Ok(Some(user)) => Ok(user),
			Ok(None) => Err(format!("User with mail {} doesn't exist.", mail).into()),
			Err(e) => Err(e)
		}
	}
	pub fn get_user_opt(&self, mail: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT mail, name, perms FROM user WHERE mail = ?1;")?;
		if let Ok(user_data) = s.query_row([mail], |r| {
			Ok(User {
				mail: r.get(0)?,
				name: r.get(1)?,
				perms: r.get(2)?
			})
		} ) {
			return Ok(Some(user_data));
		}
		Ok(None)
	}
	pub fn get_perms_or_add_with(&self, mail: &str, name: Option<&str>, create_perms: u32) -> Result<u32, Box<dyn std::error::Error>> {
		match self.get_perms_opt(mail) {
			Ok(Some(perms)) => Ok(perms),
			Ok(None) => { self.add_user(mail, name, create_perms)?; Ok(create_perms) },
			Err(e) => Err(e)
		}
	}
	pub fn user_exists(&self, mail: &str) -> Result<bool, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT EXISTS(SELECT 1 FROM user WHERE mail = ?1);")?;
		Ok(s.query_row([mail], |r| r.get(0))?)
	}
	fn create_table(&self) {
		if let Err(e) = self.con.execute("CREATE TABLE user (mail TEXT, name TEXT, perms INTEGER);", []) { tracing::error!("Failed to create user table: {}", e); }
	}
	pub fn _print(&self) -> Result<(), Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT mail, name, perms FROM user;")?;
		let ul = s.query_map([], |r| Ok(User{mail: r.get(0)?, name: r.get(1)?, perms: r.get(2)?}))?;
		for x in ul {
			let u = x?;
			tracing::info!("[{}] {}", u.perms, u.mail);
		}
		Ok(())
	}
	pub fn get(&self) -> Result<std::vec::Vec<(String, Option<String>, u32)>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT mail, name, perms FROM user;")?;
		let ul = s.query_map([], |r| Ok(User{mail: r.get(0)?, name: r.get(1)?, perms: r.get(2)?}))?;
		let mut out = Vec::new();
		for x in ul {
			let u = x?;
			out.push((u.mail, u.name, u.perms));
		}
		Ok(out)
	}
}
impl Default for UserDB {
	fn default() -> Self {
		Self::new()
	}
}
