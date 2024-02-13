use crate::article::article::Article;

const ARTICLES_DB_FILE : &str = "./articles.db";
pub struct ArticleDB {
	con: sqlite::Connection,
}

impl ArticleDB {
	pub fn new() -> Self {
		let first = !std::path::Path::new(ARTICLES_DB_FILE).exists();
		let con = sqlite::open(ARTICLES_DB_FILE).unwrap();
		let out = Self { con };
		if first {
			out.create();
		}
		out
	}
	pub fn get_no_tm(&self, id : u64) -> Result<Article, Box<dyn std::error::Error>> {
		let query = "SELECT * FROM article WHERE rowid = ?";
		let mut s = self.con.prepare(query)?;
		s.bind((1, id as i64))?;
		if let Ok(sqlite::State::Row) = s.next() {
			let title = s.read::<String, _>("title")?;
			let author = s.read::<String, _>("author")?;
			let content = s.read::<String, _>("content")?;
			let tags_str = s.read::<String, _>("tags")?;
			let tags = tags_str.split(';').map(|x| x.to_owned()).collect::<Vec<String>>();
			Ok(Article{
				id, title, author, content, tags, create_timestamp: 0u64
			})
		} else {
			Err("Article not found.".into())
		}
	}
	pub fn get(&self, id : u64) -> Result<Article, Box<dyn std::error::Error>> {
		let mut base_out = self.get_no_tm(id)?;
		let query = "SELECT * FROM article_meta WHERE id = ?";
		let mut s = self.con.prepare(query)?;
		s.bind((1, id as i64))?;
		if let Ok(sqlite::State::Row) = s.next() {
			let timestamp = s.read::<i64, _>("timestamp")? as u64;
			base_out.create_timestamp = timestamp;
			Ok(base_out)
		} else {
			Err("Article timestamp not found.".into())
		}
	}
	/// article id is ignored
	pub fn add(&self, a : &Article) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute(format!("INSERT INTO article VALUES ('{}', '{}', '{}', '{}');",
			a.title.replace('\'', "''"), a.author.replace('\'', "''"), a.content.replace('\'', "''"), a.tags.join(";").replace('\'', "''")))?;
		let id = unimplemented!(); // sqlite doesn't support last_insert_rowid()
		self.con.execute(format!("INSERT INTO article_meta VALUES ({}, {});", id, a.create_timestamp));
		Ok(())
	}

	fn create(&self) {
		if let Err(e) = self.con.execute("CREATE VIRTUAL TABLE article USING FTS5(title, author, content, tags);") { tracing::error!("Failed to create article FTS5 table: {}", e); }
		if let Err(e) = self.con.execute("CREATE TABLE article_meta (id INTEGER NOT NULL PRIMARY KEY, timestamp INTEGER NOT NULL);") { tracing::error!("Failed to create article_meta table: {}", e); }
		// maybe add another table for quicker orderby timestamp?
	}
}
