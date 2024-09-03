const IMAGE_DB_FILE : &str = "./blobs.db";
pub struct ImageDB {
	con: rusqlite::Connection,
}

impl ImageDB {
	pub fn new() -> Self {
		let first = !std::path::Path::new(IMAGE_DB_FILE).exists();
		let con = rusqlite::Connection::open(IMAGE_DB_FILE).unwrap();
		let out = Self { con };
		if first {
			out.create();
		}
		out
	}
	pub fn get(&self, id : i64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT data FROM blobs WHERE rowid=?1;")?;
		let mut out = s.query_map([id], |r| r.get(0))?;
		if let Some(x) = out.next() { Ok(x?) } else { Err("No record found in db".into()) }
	}
	pub fn add(&self, data : &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
		let hexc = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
		let mut hexs = String::new();
		for byte in data {
			hexs.push(hexc[(byte >> 4) as usize]);
			hexs.push(hexc[(byte & 0xf) as usize]);
		}
		self.con.execute(&format!("INSERT INTO blobs VALUES (X'{}')", hexs), [])?;
		Ok(self.con.last_insert_rowid())
	}
	fn create(&self) {
		if let Err(e) = self.con.execute("CREATE TABLE blobs (data BLOB NOT NULL);", []) { tracing::error!("Failed to create blobs table: {}", e); }
	}
}

impl Default for ImageDB {
	fn default() -> Self { Self::new() }
}

