use crate::article::Article;

const ARTICLES_DB_FILE : &str = "./articles.db";
pub struct ArticleDB {
	con: rusqlite::Connection,
}

impl ArticleDB {
	pub fn new() -> Self {
		let first = !std::path::Path::new(ARTICLES_DB_FILE).exists();
		let con = rusqlite::Connection::open(ARTICLES_DB_FILE).unwrap();
		let out = Self { con };
		if first {
			out.create();
		}
		out
	}
	pub fn get_no_tm(&self, id : i64) -> Result<Article, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT * FROM article WHERE rowid = ?1")?;
		Ok(s.query_row([id], |r| {
			let tags_str : String = r.get(3)?;
			Ok(Article{id, title: r.get(0)?, author: r.get(1)?, content: r.get(2)?,
				tags: tags_str.split(';').map(|x| x.to_owned()).collect::<Vec<String>>(), create_timestamp: 0u64})
		})?)
	}
	pub fn get(&self, id : i64) -> Result<Article, Box<dyn std::error::Error>> {
		let mut base_out = self.get_no_tm(id)?;
		let mut s = self.con.prepare("SELECT timestamp FROM article_meta WHERE id = ?1")?;
		base_out.create_timestamp = s.query_row([id], |r| { let timestamp : u64 = r.get(0)?; Ok(timestamp) })?;
		Ok(base_out)
	}
	pub fn get_chronol(&self, page : usize, pagesize : usize) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT id, timestamp FROM article_meta ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2;")?;
		let al = s.query_map([pagesize, page * pagesize], |r| Ok(Article{id: r.get(0)?, create_timestamp: r.get(1)?,
			tags: vec![], title: String::new(), author: String::new(), content: String::new()}))?;
		let mut out = Vec::new();
		for a in al {
			out.push(a?);
		}
		let mut s2 = self.con.prepare("SELECT * FROM article WHERE rowid = ?1;")?;
		for a in out.iter_mut() {
			s2.query_row([a.id], |r| {
				let tags_str : String = r.get(3)?;
				a.tags = tags_str.split(';').map(|x| x.to_owned()).collect::<Vec<String>>();
				a.title = r.get(0)?;
				a.author = r.get(1)?;
				a.content = r.get(2)?;
				Ok(())
			})?;
		}
		Ok(out)
	}
	/// article id is ignored, actual is returned
	pub fn add(&self, a : &Article) -> Result<i64, Box<dyn std::error::Error>> {
		self.con.execute("BEGIN TRANSACTION", [])?;
		self.con.execute("INSERT INTO article VALUES (?1, ?2, ?3, ?4)", rusqlite::params![a.title, a. author, a.content, a.tags.join(";")])?;
		let id = self.con.last_insert_rowid();
		self.con.execute("INSERT INTO article_meta VALUES (?1, ?2);", rusqlite::params![id, a.create_timestamp])?;
		self.con.execute("END TRANSACTION", [])?;
		Ok(id)
	}

	fn create(&self) {
		if let Err(e) = self.con.execute("CREATE VIRTUAL TABLE article USING FTS5(title, author, content, tags);", []) { tracing::error!("Failed to create article FTS5 table: {}", e); }
		if let Err(e) = self.con.execute("CREATE TABLE article_meta (id INTEGER NOT NULL PRIMARY KEY, timestamp INTEGER NOT NULL);", []) { tracing::error!("Failed to create article_meta table: {}", e); }
		// maybe add another table for quicker orderby timestamp?
	}
}

impl Default for ArticleDB {
	fn default() -> Self { Self::new() }
}
