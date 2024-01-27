use crate::suplovani::html_table::parse;
use crate::suplovani::supl::{Supl, Timestamp, StrSetHistory};

pub struct SuplMap {
	pub days : std::collections::BTreeMap<u64, Supl>,
	pub today : u64
}
impl SuplMap {
	pub fn new() -> Self { Self{ days: std::collections::BTreeMap::new(), today: 0 } }
}

fn get_time() -> std::time::SystemTime { std::time::SystemTime::now() }
fn find_title(data : &str) -> Option<(String, usize)> {
	let s = data.find("<h1>")? + 4;
	let e = data[s..].find("</h1>")?;
	Some((data[s..s+e].to_owned(), s+e))
}
fn find_timestamp(data : &str) -> Option<u64> {
	let mut s = data.find("ASPx.createControl(ASPxClientDateEdit")?;
	let c2 = "'rawValue':'";
	s += data[s..].find(c2)? + c2.len();
	let e = data[s..].find('\'')?;
	let out = data[s..s+e].parse::<u64>();
	if out.is_ok() { Some(unsafe { out.unwrap_unchecked() }) } else { None }
}
fn parse_table(sup : &mut Supl, tm : &Timestamp, data : &str) -> Option<usize> {
	let (t, e) = parse(&data[6..])?; // 6 = len(<table)
	if t.cells.is_empty() || t.cells[0].is_empty() || !t.cells[0][0].is_val() {
		return Some(e);
	}
	let table_id = t.cells[0][0].val_dat();
	let mci = "Nepřítomné třídy";
	let mti = "Nepřítomní učitelé";
	let mri = "Místnosti mimo provoz";
	let cci = "Změny v rozvrzích tříd";
	let cti = "Změny v rozvrzích učitelů";
	if table_id == mci || table_id == mti || table_id == mri {
		if t.cells[0].len() < 14 {
			return Some(e);
		}
		let mut ml : [std::vec::Vec<String>; 13] = Default::default();
		for row in t.cells.iter().skip(1) {
			if !row[0].is_val() { continue; }
			let obj = row[0].val_dat();
			for i in 0..13 {
				if row[i+1].is_val() && !row[i+1].val_dat().is_empty() && row[i+1].val_dat().trim() != "-" {
					ml[i].push(obj.to_owned());
				}
			}
		}
		if table_id == mci { for (m, h) in ml.iter().zip(sup.hours.iter_mut()) { h.missing_classes.update(m.clone(), *tm); } }
		else if table_id == mti { for (m, h) in ml.iter().zip(sup.hours.iter_mut()) { h.missing_teachers.update(m.clone(), *tm); } }
		else if table_id == mri { for (m, h) in ml.iter().zip(sup.hours.iter_mut()) { h.missing_rooms.update(m.clone(), *tm); } }
	} else if table_id == cci {
		if t.cells[0].len() < 2 {
			return Some(e);
		}
		let mut cl : [std::collections::HashMap<String, std::vec::Vec<String>>; 13] = Default::default();
		for row in t.cells.iter().skip(1) {
			if !row[0].is_val() || row[1].is_val() { continue; }
			let cls = row[0].val_dat();
			let t2 = row[1].tab_dat();
			if t2.cells.is_empty() || t2.cells[0].is_empty() { continue; }
			for r2 in &t2.cells {
				if !r2[0].is_val() || (r2.len() >= 7 && (!r2[0].is_val() || !r2[1].is_val() || !r2[2].is_val() || !r2[3].is_val() || !r2[4].is_val()
					|| !r2[5].is_val() || !r2[6].is_val())) { continue; }
				let phi = r2[0].val_dat().parse::<usize>();
				if r2.len() < 7 || phi.is_err() || unsafe{ phi.clone().unwrap_unchecked() } >= 13 { // why do I need to clone :(
					let re = regex::Regex::new(r"([0-9]+)\. (hod|les).").unwrap();
					if let Some(m) = re.captures(r2[0].val_dat()) {
						if let Ok(hi) = m[1].parse::<usize>() { if hi < 13 {
							let cs = format!("!{}|||||", r2[0].val_dat());
							cl[hi].entry(cls.to_owned()).or_default().push(cs);
						} }
					}
				} else {
					let subj = r2[1].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let group = r2[2].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let room = r2[3].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let change = r2[4].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let teacher = r2[5].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let note = r2[6].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let cs = format!("{}|{}|{}|{}|{}|{}", change, group, subj, teacher, room, note);
					cl[unsafe{ phi.unwrap_unchecked() }].entry(cls.to_owned()).or_default().push(cs);
				}
			}
		}
		for (c, h) in cl.iter().zip(sup.hours.iter_mut()) {
			for (cls, ch) in c {
				if !ch.is_empty() {
					h.class_changes.entry(cls.to_owned()).or_insert_with(StrSetHistory::new).update(ch.clone(), *tm);
				}
			}
			for (k, v) in h.class_changes.iter_mut() {
				if !c.contains_key(k) {
					v.update(vec![], *tm);
				}
			}
		}
	} else if table_id == cti {
		if t.cells[0].len() < 2 {
			return Some(e);
		}
		let mut cl : [std::collections::HashMap<String, std::vec::Vec<String>>; 13] = Default::default();
		for row in t.cells.iter().skip(1) {
			if !row[0].is_val() || row[1].is_val() { continue; }
			let tch = row[0].val_dat();
			let t2 = row[1].tab_dat();
			if t2.cells.is_empty() || t2.cells[0].is_empty() { continue; }
			for r2 in &t2.cells {
				if !r2[0].is_val() || (r2.len() >= 6 && (!r2[1].is_val() || !r2[2].is_val() || !r2[3].is_val() || !r2[4].is_val() || !r2[5].is_val())) { continue; }
				let phi = r2[0].val_dat().parse::<usize>();
				if r2.len() < 6 || phi.is_err() || unsafe{ phi.clone().unwrap_unchecked() } >= 13 { // why do I need to clone :(
					let re = regex::Regex::new(r"([0-9]+)\. (hod|les).").unwrap();
					if let Some(m) = re.captures(r2[0].val_dat()) {
						if let Ok(hi) = m[1].parse::<usize>() { if hi < 13 {
							let cs = format!("!{}||||", r2[0].val_dat());
							cl[hi].entry(tch.to_owned()).or_default().push(cs);
						} }
					}
				} else {
					let change = r2[1].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let subj = r2[2].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let class_group = r2[3].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let room = r2[4].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let note = r2[5].val_dat().split_ascii_whitespace().collect::<Vec<_>>().join(" ");
					let cs = format!("{}|{}|{}|{}|{}", change, class_group, subj, room, note);
					cl[unsafe{ phi.unwrap_unchecked() }].entry(tch.to_owned()).or_default().push(cs);
				}
			}
		}
		for (c, h) in cl.iter().zip(sup.hours.iter_mut()) {
			for (cls, ch) in c {
				if !ch.is_empty() {
					h.teacher_changes.entry(cls.to_owned()).or_insert_with(StrSetHistory::new).update(ch.clone(), *tm);
				}
			}
			for (k, v) in h.teacher_changes.iter_mut() {
				if !c.contains_key(k) {
					v.update(vec![], *tm);
				}
			}
		}
	} else {
		tracing::warn!("Unknown table: [{}]", table_id);
	}
	Some(e)
}
fn iter_tables(sup : &mut Supl, tm : &Timestamp, data : &str) {
	let mut s : usize = 0;
	let mut s_w = data[s..].find("<table");
	while s_w.is_some() {
		s += unsafe { s_w.unwrap_unchecked() };
		if let Some(e) = parse_table(sup, tm, &data[s..]) {
			s += e;
			s_w = data[s..].find("<table");
		} else {
			break;
		}
	}
}
fn parse_data(sup : &mut Supl, tm : &Timestamp, data : &str) {
	if let Some(title) = find_title(data) {
		tracing::info!("parsing [{}]", title.0);
		iter_tables(sup, tm, &data[title.1..]);
	} else {
		tracing::error!("Could not find title!");
	}
}
fn handle_resp(sm : &mut SuplMap, res_w : Result<reqwest::blocking::Response, reqwest::Error>) -> Option<u64> {
	if res_w.is_err() {
		tracing::error!("Supl request failed!");
		return None;
	}
	let res = unsafe { res_w.unwrap_unchecked() };
	if res.status() != 200 {
		tracing::error!("Supl request returned non-OK code ({})!", res.status());
		return None;
	}
	let bytes = res.bytes();
	if bytes.is_err() {
		tracing::error!("Supl couldn't get bytes from response!");
		return None;
	}
	if let Ok(data) = std::str::from_utf8(&unsafe { bytes.unwrap_unchecked() }) {
		let curr_time = get_time();
		let res = find_timestamp(data)?;
		let day_id = res / 1000 / 3600 / 24;
		parse_data(sm.days.entry(day_id).or_insert_with(Supl::new), &curr_time, data);
		Some(res)
	} else {
		tracing::error!("Supl - bad data format!");
		None
	}
}
pub fn fetch_data(sm : &mut SuplMap) {
	let url = "https://dochazka.gjk.cz/next/zmeny.aspx";
	let client = reqwest::blocking::Client::new();
	let curr_time_w = handle_resp(sm, client.get(url).send());
	if curr_time_w.is_none() {
		return;
	}	
	let curr_time = unsafe { curr_time_w.unwrap_unchecked() };
	sm.today = curr_time / 1000 / 24 / 3600;
	let day_add = 24 * 3600 * 1000;
	for i in 1..3 {
		let payload = format!("__EVENTTARGET=DateEdit&__EVENTARGUMENT=&DateEdit%24State=%7B%26quot%3BrawValue%26quot%3B%3A%26quot%3B{}\
			%26quot%3B%2C%26quot%3BuseMinDateInsteadOfNull%26quot%3B%3Afalse%7D&DateEdit=&DateEdit%24DDDState=%7B%26quot%3BwindowsState%26\
			quot%3B%3A%26quot%3B0%3A0%3A-1%3A75%3A70%3A1%3A%3A%3A1%3A0%3A0%3A0%26quot%3B%7D&DateEdit%24DDD%24C=%7B%7D&FilterDropDown_VI=1&\
			FilterDropDown=cel%C3%A1+%C5%A1kola&FilterDropDown%24DDDState=%7B%26quot%3BwindowsState%26quot%3B%3A%26quot%3B0%3A0%3A-1%3A0%3\
			A0%3A0%3A-10000%3A-10000%3A1%3A0%3A0%3A0%26quot%3B%7D&FilterDropDown%24DDD%24L%24State=%7B%7D&FilterDropDown%24DDD%24L=1&DXScript=&DXCss=", curr_time + i*day_add);
		if handle_resp(sm, client.post(url).
				header("Content-type", "application/x-www-form-urlencoded").
				body(payload).send()).is_none() {
		}	
	}
}
