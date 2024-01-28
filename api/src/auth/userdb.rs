

const USERDB_FILE : &str = "./userdb.db";
pub struct UserDB {
	con : sqlite::Connection
}

impl UserDB {
	pub fn new() -> Self {
		let first = !std::path::Path::new(USERDB_FILE).exists();
		let con = sqlite::open(USERDB_FILE).unwrap();
		let out = Self { con };
		if first {
			out.create_table();
		}
		out
	}

	pub fn add_user(&self, mail : &str, perms : u32) -> Result<(), Box<dyn std::error::Error>> {
		let q = format!("INSERT INTO user VALUES ('{}', {});", mail.replace("'", "''"), perms);
		self.con.execute(q)?;
		Ok(())
	}
	pub fn remove_user(&self, mail : &str) -> Result<(), Box<dyn std::error::Error>> {
		let q = format!("DELETE FROM user WHERE mail = '{}';", mail.replace("'", "''"));
		self.con.execute(q)?;
		Ok(())
	}
	pub fn update_perms(&self, mail : &str, perms : u32) -> Result<(), Box<dyn std::error::Error>> {
		let q = format!("UPDATE user SET perms = {} WHERE mail = '{}';", perms, mail.replace("'", "''"));
		self.con.execute(q)?;
		Ok(())
	}
	pub fn get_perms(&self, mail : &str) -> Result<u32, Box<dyn std::error::Error>> {
		let q = "SELECT * FROM user WHERE mail = ?;";
		let mut s = self.con.prepare(q)?;
		s.bind((1, mail))?;
		if let Ok(sqlite::State::Row) = s.next() {
			return Ok(s.read::<i64, _>("perms")? as u32);
		}
		Err(format!("User with mail {} doesn't exist.", mail).into())
	}
	pub fn get_perms_opt(&self, mail : &str) -> Result<Option<u32>, Box<dyn std::error::Error>> {
		let q = "SELECT * FROM user WHERE mail = ?;";
		let mut s = self.con.prepare(q)?;
		s.bind((1, mail))?;
		if let Ok(sqlite::State::Row) = s.next() {
			return Ok(Some(s.read::<i64, _>("perms")? as u32));
		}
		Ok(None)
	}
	pub fn get_perms_or_add_with(&self, mail : &str, create_perms : u32) -> Result<u32, Box<dyn std::error::Error>> {
		let q = "SELECT * FROM user WHERE mail = ?;";
		let mut s = self.con.prepare(q)?;
		s.bind((1, mail))?;
		if let Ok(sqlite::State::Row) = s.next() {
			return Ok(s.read::<i64, _>("perms")? as u32);
		}
		self.add_user(mail, create_perms)?;
		Ok(create_perms)
	}
	pub fn user_exists(&self, mail : &str) -> Result<bool, Box<dyn std::error::Error>> {
		let q = "SELECT * FROM user WHERE mail = ?;";
		let mut s = self.con.prepare(q)?;
		s.bind((1, mail))?;
		if let Ok(sqlite::State::Row) = s.next() {
			return Ok(true);
		}
		Ok(false)
	}
	fn create_table(&self) {
		if let Some(e) = self.con.execute("CREATE TABLE user (mail TEXT, perms INTEGER);").err() {
			tracing::error!("Failed to create user table: {}", e);
		}
	}
	pub fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
		self.con.iterate("SELECT * FROM user;", |data| {
			tracing::info!("[{}] {}", data.iter().find(|(k, _)| k == &"perms").unwrap().1.unwrap(), 
				data.iter().find(|(k, _)| k == &"mail").unwrap().1.unwrap());
			true
		})?;
		Ok(())
	}
}
impl Default for UserDB {
	fn default() -> Self {
		Self::new()
	}
}
