pub mod html_table;
pub mod supl;
pub mod archive;
pub mod supl_driver;

pub struct Suplovani {
	sm : std::option::Option<supl_driver::SuplMap>,
	update_thr : std::option::Option<std::thread::JoinHandle<()>>,
	json : std::sync::Arc<std::sync::Mutex<String>>,
}

impl Suplovani {
	pub fn new() -> Self {
		Self {
			sm: Some(supl_driver::SuplMap::new()),
			update_thr: None,
			json: std::sync::Arc::new(std::sync::Mutex::new(String::from("{\"curr_day\":null,\"days\":null}"))),
		}
	}
	pub fn prepare() {
		let _ = std::fs::create_dir_all("./suplbackup/");
	}
	pub fn load(&mut self) {
		for f in std::fs::read_dir("./suplbackup/").unwrap() {
			let name = f.unwrap().file_name().into_string().unwrap();
			tracing::info!("loading supl backup {}", name);
			let data  = std::fs::read(format!("./suplbackup/{}", name)).unwrap();
			let mut di = data.iter();
			let dayid = name[4..].parse::<u64>().unwrap();
			let sm = self.sm.as_mut().unwrap();
			sm.days.insert(dayid, supl::Supl::new());
			archive::deserialize_day(&mut di, sm.days.get_mut(&dayid).unwrap());
		}
	}
	pub fn start_thread(&mut self, interval : std::time::Duration) {
		let smm = self.sm.take();
		let jsonr = self.json.clone();
		self.update_thr = Some(std::thread::spawn(move || {
			let mut sm = smm.unwrap();
			loop {
				tracing::info!("suplovani refresh...");
				supl_driver::fetch_data(&mut sm);
				archive::save_and_filter_data(&mut sm);
				let mut ad = String::from("{\"curr_day\":");
				ad.push_str(&sm.today.to_string());
				ad.push_str(",\"days\":[");
				for (day, supl) in sm.days.iter() {
					let mut dd = String::from("{\"id\":");
					dd.push_str(&day.to_string());
					dd.push_str(",\"hours\":[");
					for h in supl.hours.iter() {
						dd.push_str(&h.serialize());
						dd.push(',');
					}
					dd.pop();
					dd.push_str("]}");
					ad.push_str(&dd);
					ad.push(',');
				}
				if !sm.days.is_empty() { ad.pop(); }
				ad.push_str("]}");
				*jsonr.lock().unwrap() = ad;
				std::thread::sleep(interval);
			}
		}));
	}
	pub fn get_json(&self) -> String {
		(*self.json.lock().unwrap()).clone()
	}
}
