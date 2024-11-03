use rusqlite::OptionalExtension;
use crate::article::Article;
use crate::routes::article::edit::EditArticle;

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
	pub fn get_no_meta(&self, id : i64) -> Result<Option<Article>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT * FROM article WHERE rowid = ?1")?;
		Ok(s.query_row([id], |r| {
			let tags_str : String = r.get(4)?;
			Ok(Article{id, title: r.get(0)?, author_email: r.get(1)?, author_name: r.get(2)?, content: r.get(3)?,
				tags: tags_str.split(';').map(|x| x.to_owned()).collect::<Vec<String>>(),
				create_timestamp: 0, thumbnail_id: 0 })
		}).optional()?)
	}
	pub fn get(&self, id : i64) -> Result<Option<Article>, Box<dyn std::error::Error>> {
		if let Some(mut base_out) = self.get_no_meta(id)? {
			let mut s = self.con.prepare("SELECT timestamp, thumbnail FROM article_meta WHERE id = ?1")?;
			let meta = s.query_row([id], |r| { let timestamp : u64 = r.get(0)?; let thumbnail : u64 = r.get(0)?; Ok((timestamp, thumbnail)) })?;
			base_out.create_timestamp = meta.0;
			base_out.thumbnail_id = meta.1;
			return Ok(Some(base_out));
		}
		Ok(None)
	}
	pub fn get_chronol(&self, page : usize, pagesize : usize) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
		let mut s = self.con.prepare("SELECT id, timestamp, thumbnail FROM article_meta ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2;")?;
		let al = s.query_map([pagesize, page * pagesize], |r| Ok(Article{
			id: r.get(0)?, create_timestamp: r.get(1)?, thumbnail_id: r.get(2)?,
			tags: vec![], title: String::new(), author_email: String::new(), author_name: None, content: String::new()}))?;
		let mut out = Vec::new();
		for a in al {
			out.push(a?);
		}
		let mut s2 = self.con.prepare("SELECT * FROM article WHERE rowid = ?1;")?;
		for a in out.iter_mut() {
			s2.query_row([a.id], |r| {
				let tags_str : String = r.get(4)?;
				a.tags = tags_str.split(';').map(|x| x.to_owned()).collect::<Vec<String>>();
				a.title = r.get(0)?;
				a.author_email = r.get(1)?;
				a.author_name = r.get(2)?;
				a.content = r.get(3)?;
				Ok(())
			})?;
		}
		Ok(out)
	}
	/// article id is ignored, actual is returned
	pub fn add(&self, a : &Article) -> Result<i64, Box<dyn std::error::Error>> {
		self.con.execute("BEGIN TRANSACTION", [])?;
		self.con.execute("INSERT INTO article VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![a.title, a.author_email, a.author_name, a.content, a.tags.join(";")])?;
		let id = self.con.last_insert_rowid();
		self.con.execute("INSERT INTO article_meta VALUES (?1, ?2, ?3);", rusqlite::params![id, a.create_timestamp, a.thumbnail_id])?;
		self.con.execute("END TRANSACTION", [])?;
		Ok(id)
	}

	/// Edits the old article with new article data
	///
	/// Only edits: Title, content, tags, thumbnail id
	///
	/// The rest is ignored
	pub fn edit(&self, id: i64, a: &EditArticle) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("BEGIN TRANSACTION", [])?;
		self.con.execute("UPDATE article SET title = ?1, content = ?2, tags = ?3 WHERE rowid = ?4;", rusqlite::params![a.title, a.content, a.tags.join(";"), id])?;
		self.con.execute("UPDATE article_meta SET thumbnail = ?1 WHERE id = ?2;", rusqlite::params![a.thumbnail_id, id])?;
		self.con.execute("END TRANSACTION", [])?;
		Ok(())
	}

	/// Deletes the article
	pub fn delete(&self, id: i64) -> Result<i64, Box<dyn std::error::Error>> {
		self.con.execute("BEGIN TRANSACTION", [])?;
		self.con.execute("DELETE FROM article WHERE rowid = ?1;", rusqlite::params![id])?;
		self.con.execute("DELETE FROM article_meta WHERE id = ?1;", rusqlite::params![id])?;
		self.con.execute("END TRANSACTION", [])?;
		Ok(id)
	}

	pub fn rename_author(&self, author_mail: &str, author_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
		self.con.execute("UPDATE article SET author_name = ?1 WHERE author_email = ?2;", rusqlite::params![author_name, author_mail])?;
		Ok(())
	}

	fn create(&self) {
		if let Err(e) = self.con.execute("CREATE VIRTUAL TABLE article USING FTS5(title, author_email, author_name, content, tags);", []) { tracing::error!("Failed to create article FTS5 table: {}", e); }
		if let Err(e) = self.con.execute("CREATE TABLE article_meta (id INTEGER NOT NULL PRIMARY KEY, timestamp INTEGER NOT NULL, thumbnail INTEGER NOT NULL);", []) { tracing::error!("Failed to create article_meta table: {}", e); }
		// maybe add another table for quicker orderby timestamp?
	}
}

impl Default for ArticleDB {
	fn default() -> Self { Self::new() }
}
