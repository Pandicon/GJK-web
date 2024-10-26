pub mod db;
pub mod imagedb;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
	#[serde(skip_deserializing)]
	pub id: i64,
	pub title: String,
	#[serde(skip_deserializing)]
	pub author_email: String,
	pub content: String,
	pub tags: Vec<String>,
	#[serde(skip_deserializing)]
	pub create_timestamp: u64,
	pub thumbnail_id: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArticleWithAuthorNameWithoutAuthorEmail {
	pub id: i64,
	pub title: String,
	pub author_name: Option<String>,
	pub content: String,
	pub tags: Vec<String>,
	pub create_timestamp: u64,
	pub thumbnail_id: u64,
}

impl ArticleWithAuthorNameWithoutAuthorEmail {
	pub fn from_article(article: Article, author_name: Option<String>) -> Self {
		Self {
			id: article.id,
			title: article.title,
			author_name,
			content: article.content,
			tags: article.tags,
			create_timestamp: article.create_timestamp,
			thumbnail_id: article.thumbnail_id,
		}
	}
}