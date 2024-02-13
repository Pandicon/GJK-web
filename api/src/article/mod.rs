pub mod db;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
	#[serde(skip_deserializing)]
	pub id: i64,
	pub title: String,
	pub author: String,
	pub content: String,
	pub tags: Vec<String>,
	#[serde(skip_deserializing)]
	pub create_timestamp: u64,
}
