pub trait Cell {
	fn is_val(&self) -> bool;
	fn val_dat(&self) -> &str;
	fn tab_dat(&self) -> &Table;
}
pub struct Value {
	pub data : String
}
pub struct Table {
	pub cells : std::vec::Vec<std::vec::Vec<std::boxed::Box<dyn Cell>>>
}
pub fn parse(data : &str) -> Option<(Table, usize)> {
	let mut e = data.find("</table>")?;
	if let Some(e2) = data.find("<table") { if e2 < e { // let Some(x) doesn't like &&
		let mut c : u32 = 1;
		let mut i : usize = 0;
		while c > 0 {
			let e1 = data[i..].find("<table");
			let e2 = data[i..].find("</table>")?;
			if e1.is_none() || unsafe { e1.unwrap_unchecked() } > e2 { i += e2+8; c-=1; }
			else { i += unsafe { e1.unwrap_unchecked() }+7; c+=1; }
		}
		e = i;
	}}
	let mut t = Table{ cells: vec![] };
	let mut i : usize = 0;
	let mut sz : usize = 0;
	while let Some(rs) = data[i..e].find("<tr") {
		t.cells.push(vec![]);
		let mut j = i+rs+3;
		loop {
			let ftre = data[j..e].find("</tr>")?;
			let ftd = data[j..e].find("<td").unwrap_or(ftre+1);
			let fth = data[j..e].find("<th").unwrap_or(ftre+1);
			if ftre < ftd && ftre < fth { i = j + ftre; break; }
			if ftd < fth {
				j += ftd;
				let ftde = data[j..e].find("</td>")?;
				let ftb = data[j..e].find("<table").unwrap_or(ftde+1);
				if ftde < ftb {
					let d = &data[j+data[j..e].find('>').unwrap_or(0)+1..j+ftde].trim();
					unsafe { t.cells.last_mut().unwrap_unchecked() }.push(std::boxed::Box::new(Value{data: d.to_string()}));
					j += ftde;
				} else {
					j += ftb;
					let (t2, e2) = parse(&data[j+6..e])?; // 6 = len(<table)
					unsafe { t.cells.last_mut().unwrap_unchecked() }.push(std::boxed::Box::new(t2));
					j += e2;
					j += data[j..e].find("</td>")?;
				}
			} else {
				j += fth;
				let fthe = data[j..e].find("</th>")?;
				let ftb = data[j..e].find("<table").unwrap_or(fthe+1);
				if fthe < ftb {
					let d = &data[j+data[j..e].find('>').unwrap_or(0)+1..j+fthe].trim();
					unsafe { t.cells.last_mut().unwrap_unchecked() }.push(std::boxed::Box::new(Value{data: d.to_string()}));
					j += fthe;
				} else {
					j += ftb;
					let (t2, e2) = parse(&data[j..e])?;
					unsafe { t.cells.last_mut().unwrap_unchecked() }.push(std::boxed::Box::new(t2));
					j += e2;
					j += data[j..e].find("</th>")?;
				}
			}
		}
		if sz < unsafe { t.cells.last().unwrap_unchecked() }.len() { sz = unsafe { t.cells.last().unwrap_unchecked() }.len(); }
	}
	for row in t.cells.iter_mut() {
		while row.len() < sz {
			row.push(std::boxed::Box::new(Value{data:"".to_owned()}));
		}
	}
	Some((t, e))
}

impl Cell for Value {
	fn is_val(&self) -> bool { true }
	fn val_dat(&self) -> &str { &self.data }
	fn tab_dat(&self) -> &Table { panic!("tab_dat call on value") }
}
impl Cell for Table {
	fn is_val(&self) -> bool { false }
	fn val_dat(&self) -> &str { panic!("val_dat call on table") }
	fn tab_dat(&self) -> &Table { self }
}
