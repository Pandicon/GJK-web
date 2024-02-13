pub struct Article {
	pub id: u64,
	pub title: String,
	pub author: String,
	pub content: String,
	pub tags: Vec<String>,
	pub create_timestamp: u64,
}
