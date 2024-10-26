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

