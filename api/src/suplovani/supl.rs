pub type Timestamp = std::time::SystemTime;
pub struct History<DT> {
	pub deltas : std::vec::Vec<DT>,
	pub timestamps: std::vec::Vec<Timestamp>
}
pub type StrSetHistory = History<(bool, String)>;
type ChangesHist = std::collections::BTreeMap<String, StrSetHistory>;
pub struct Hour {
	pub missing_classes : StrSetHistory,
	pub missing_teachers : StrSetHistory,
	pub missing_rooms : StrSetHistory,
	pub class_changes : ChangesHist,
	pub teacher_changes : ChangesHist
}
pub struct Supl {
	pub hours : [Hour; 13]
}

impl<DT> History<DT> {
	pub fn new() -> Self { Self { deltas : vec![], timestamps : vec![] } }
}
impl History<(bool, String)> {
	pub fn update(&mut self, nv : std::vec::Vec<String>, tm : Timestamp) -> usize {
		let s = self.deltas.len();
		let mut done = std::collections::HashSet::new();
		let mut v = std::collections::HashSet::new();
		for i in self.deltas.iter().rev() {
			if !done.contains(&i.1) && i.0 {
				v.insert(i.1.clone());
			}
			done.insert(i.1.clone());
		}
		for i in &nv {
			if v.contains(i) {
				v.remove(i);
			} else {
				self.deltas.push((true, i.to_string()));
				self.timestamps.push(tm);
			}
		}
		for i in v {
			self.deltas.push((false, i.to_string()));
			self.timestamps.push(tm);
		}
		s
	}
	pub fn serialize(&self, name : &str) -> String {
		let mut out = String::from("\"");
		out.push_str(name);
		out.push_str("\":[");
		for (d, t) in self.deltas.iter().zip(self.timestamps.iter()) {
			out.push_str("{\"value\":\"");
			out.push_str(&d.1);
			out.push_str("\",\"added\":");
			out.push_str(if d.0 { "true" } else { "false" });
			out.push_str(",\"time\":\"");
			let tm : chrono::DateTime<chrono::Local> = (*t).into();
			out.push_str(&format!("{}", tm.format("%a %-d.%-m. %-H:%M")));
			out.push_str(&format!("\",\"ut\":\"{}\"", t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
			out.push_str("},");
		}
		if !self.deltas.is_empty() { out.pop(); }
		out.push_str("],");
		out
	}
}
impl Hour {
	pub fn new() -> Self { Self {
		missing_classes : StrSetHistory::new(),
		missing_teachers : StrSetHistory::new(),
		missing_rooms : StrSetHistory::new(),
		class_changes : ChangesHist::new(),
		teacher_changes : ChangesHist::new()
	} }
	pub fn serialize(&self) -> String {
		let mut out = String::from("{");
		out.push_str(&self.missing_classes.serialize("missing_classes"));
		out.push_str(&self.missing_teachers.serialize("missing_teachers"));
		out.push_str(&self.missing_rooms.serialize("missing_rooms"));
		out.push_str("\"class_changes\":{");
		for (k, v) in self.class_changes.iter() {
			out.push('"');
			out.push_str(k);
			out.push_str("\":{");
			out.push_str(&v.serialize("changes"));
			out.pop();
			out.push_str("},");
		}
		if !self.class_changes.is_empty() { out.pop(); }
		out.push_str("},\"teacher_changes\":{");
		for (k, v) in self.teacher_changes.iter() {
			out.push('"');
			out.push_str(k);
			out.push_str("\":{");
			out.push_str(&v.serialize("changes"));
			out.pop();
			out.push_str("},");
		}
		if !self.teacher_changes.is_empty() { out.pop(); }
		out.push_str("}}");
		out
	}
}
impl Supl {
	pub fn new() -> Self { Self { hours : [Hour::new(), Hour::new(), Hour::new(), Hour::new(), Hour::new(), Hour::new(), Hour::new(), Hour::new(), Hour::new(),
		Hour::new(), Hour::new(), Hour::new(), Hour::new()] } }
}
