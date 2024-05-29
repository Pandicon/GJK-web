use crate::suplovani::supl::{History, StrSetHistory, Supl};
use crate::suplovani::supl_driver::SuplMap;

fn serialize_history(out : &mut std::vec::Vec<u8>, hist : &StrSetHistory) {
	out.extend((hist.deltas.len() as u16).to_le_bytes());
	for ((added, val), time) in hist.deltas.iter().zip(hist.timestamps.iter()) {
		out.push(*added as u8);
		out.extend(time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_le_bytes());
		if val.len() > 0x7fff {
			tracing::warn!("Value too long ({} bytes) for correct serialization!", val.len());
		}
		if val.len() > 127 {
			out.push((val.len() >> 8) as u8 | 128);
		}
		out.push(val.len() as u8);
		out.extend(val.as_bytes());
	}
}
fn deserialize_str<'a, I>(data : &mut I, len : usize) -> String where I : Iterator<Item=&'a u8> {
	let mut bytes = std::vec::Vec::with_capacity(len);
	for _ in 0..len {
		bytes.push(*data.next().unwrap());
	}
	std::str::from_utf8(bytes.as_slice()).unwrap().to_owned()
}
fn deserialize_history<'a, I>(data : &mut I, hist : &mut StrSetHistory) where I : Iterator<Item=&'a u8> {
	let len = *data.next().unwrap() as u16 | (*data.next().unwrap() as u16) << 8;
	for _ in 0..len {
		let added = *data.next().unwrap() != 0;
		let timestamp = (*data.next().unwrap() as u64) |
			(*data.next().unwrap() as u64) << 8 |
			(*data.next().unwrap() as u64) << 16 |
			(*data.next().unwrap() as u64) << 24 |
			(*data.next().unwrap() as u64) << 32 |
			(*data.next().unwrap() as u64) << 40 |
			(*data.next().unwrap() as u64) << 48 |
			(*data.next().unwrap() as u64) << 56;
		let mut val_len = *data.next().unwrap() as u16;
		if (val_len & 128) != 0 {
			val_len = (val_len & !128) << 8 | (*data.next().unwrap() as u16);
		}
		let val = deserialize_str(data, val_len as usize);
		let tm = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
		hist.deltas.push((added, val.clone()));
		hist.timestamps.push(tm);
	}
}
fn serialize_day(data : &Supl) -> std::vec::Vec<u8> {
	let mut out = vec![];
	for hour in data.hours.iter() {
		let mut hout = vec![];
		serialize_history(&mut hout, &hour.missing_classes);
		serialize_history(&mut hout, &hour.missing_teachers);
		serialize_history(&mut hout, &hour.missing_rooms);
		hout.push(hour.class_changes.len() as u8);
		for (cls, changes) in hour.class_changes.iter() {
			hout.push(cls.len() as u8);
			hout.extend(cls.bytes());
			serialize_history(&mut hout, changes);
		}
		hout.push(hour.teacher_changes.len() as u8);
		for (teacher, changes) in hour.teacher_changes.iter() {
			hout.push(teacher.len() as u8);
			hout.extend(teacher.bytes());
			serialize_history(&mut hout, changes);
		}
		out.extend((hout.len() as u16).to_le_bytes());
		out.extend(hout);
	}
	out
}
pub fn deserialize_day<'a, I>(data : &mut I, sup : &mut Supl) where I : Iterator<Item=&'a u8> {
	for hour in sup.hours.iter_mut() {
		data.next(); data.next();
		deserialize_history(data, &mut hour.missing_classes);
		deserialize_history(data, &mut hour.missing_teachers);
		deserialize_history(data, &mut hour.missing_rooms);
		let ccc = *data.next().unwrap();
		for _ in 0..ccc {
			let cls_len = *data.next().unwrap();
			let cls = deserialize_str(data, cls_len as usize);
			hour.class_changes.entry(cls.clone()).or_insert(History::new());
			deserialize_history(data, hour.class_changes.get_mut(&cls).unwrap());
		}
		let tcc = *data.next().unwrap();
		for _ in 0..tcc {
			let teacher_len = *data.next().unwrap();
			let teacher = deserialize_str(data, teacher_len as usize);
			hour.teacher_changes.entry(teacher.clone()).or_insert(History::new());
			deserialize_history(data, hour.teacher_changes.get_mut(&teacher).unwrap());
		}
	}
}
pub fn archive(day_id : u64, day_data : &Supl) {
	// archive files contain this:
	// ---------- header section --------------
	// for each slot (so 10 times) {
	//     little_endian_u16 start_location
	//     little_endian_u16 block_length
	// }
	// ---------- body section ----------------
	// data serialized through serialize_day function

	let batchid = day_id / 1000;
	let batchdir = format!("./suplarchive/batch_{}/", batchid);
	let fileid = day_id / 10 % 100;
	let file = format!("{}file_{}", batchdir, fileid);
	let _ = std::fs::create_dir_all(batchdir);

	let header_length = 40;
	let header_loc = day_id % 10 * 4;
	let mut data = vec![];
	if let Ok(prev) = std::fs::read(file.clone()) {
		data = prev;
		let old_len = (data[(header_loc + 3) as usize] as u16) << 8 | (data[(header_loc + 2) as usize] as u16);
		if old_len != 0 {
			let old_start = (data[(header_loc + 1) as usize] as u16) << 8 | (data[header_loc as usize] as u16);
			data.drain((old_start+header_length) as usize..((old_start+header_length+old_len) as usize));
			for i in 0..10 {
				let start = (data[i + 1] as u16) << 8 | (data[i] as u16);
				if start > old_start {
					let shifted = start - old_len;
					data[i] = shifted as u8;
				}
			}
		}
	} else {
		data.resize(40, 0);
	}
	let dd = serialize_day(day_data);
	let start = (data.len()-(header_length as usize)) as u16;
	let len = dd.len() as u16;
	data[header_loc as usize] = start as u8;
	data[(header_loc + 1) as usize] = (start >> 8) as u8;
	data[(header_loc + 2) as usize] = len as u8;
	data[(header_loc + 3) as usize] = (len >> 8) as u8;
	data.extend(dd);
	let _ = std::fs::write(file, data);
}
pub fn save_and_filter_data(sm : &mut SuplMap) {
	for (day_id, day_data) in sm.days.iter() {
		let quick_backup_file = format!("./suplbackup/day_{}", day_id);
		if day_id + 7 > sm.today {
			let _ = std::fs::write(quick_backup_file, serialize_day(day_data));
		} else {
			archive(*day_id, day_data);
			let _ = std::fs::remove_file(quick_backup_file);
		}
	}
	sm.days.retain(|&day_id, _| day_id + 7 > sm.today);
}
