use rand::{Rng, SeedableRng};
use sha::utils::Digest;

type Token = [u8; 48];
pub struct TokenStorage {
	map : std::sync::Mutex<std::collections::HashMap<Token, String>>,
	rng : std::sync::Mutex<rand::rngs::StdRng>
}
impl TokenStorage {
	pub fn new() -> Self {
		Self{ map: std::sync::Mutex::new(std::collections::HashMap::new()), rng: std::sync::Mutex::new(rand::rngs::StdRng::from_entropy()) }
	}
	fn gen_token(&self, mail : &str) -> Token {
		let mut r = [0u8; 32];
		let tm = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		let mut rlg = self.rng.lock().unwrap();
		rlg.fill(&mut r);
		let mut out = sha::sha1::Sha1::default().digest(mail.as_bytes()).to_bytes_len(8);
		out.extend_from_slice(&tm.to_le_bytes());
		out.extend_from_slice(&r);
		out.try_into().unwrap()
	}
	pub fn create(&self, mail : &str) -> Token {
		let t = self.gen_token(mail);
		self.map.lock().unwrap().insert(t, mail.to_owned());
		t
	}
	pub fn get(&self, token : &Token) -> Option<String> {
		let lg = self.map.lock().unwrap();
		let o = lg.get(token);
		if o.is_none() { None }
		else { Some(unsafe{o.unwrap_unchecked()}.clone()) }
	}
	pub fn remove(&self, token : &Token) {
		let mut lg = self.map.lock().unwrap();
		lg.remove(token);
	}
	pub fn filter(&self, max_age : u64) {
		let tm = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		self.map.lock().unwrap().retain(|&token, _| {
			let tmb = &token[8..16];
			let timestamp = u64::from_le_bytes([tmb[0], tmb[1], tmb[2], tmb[3], tmb[4], tmb[5], tmb[6], tmb[7]]);
			timestamp + max_age >= tm
		});
	}
}
impl Default for TokenStorage {
	fn default() -> Self {
		Self::new()
	}
}
pub fn token_to_str(t : &Token) -> String {
	let b64c = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
	let mut out = String::with_capacity(64);
	for i in 0..16 {
		let group = (t[3*i] as u32) << 16 | (t[3*i+1] as u32) << 8 | (t[3*i+2] as u32);
		out.push(b64c[((group >> 18) & 0x3f) as usize] as char);
		out.push(b64c[((group >> 12) & 0x3f) as usize] as char);
		out.push(b64c[((group >> 6) & 0x3f) as usize] as char);
		out.push(b64c[(group & 0x3f) as usize] as char);
	}
	out
}
pub fn token_from_str(s : &str) -> Result<Token, &str> {
	let b = s.as_bytes();
	if b.len() != 64 { return Err("wrong string len"); }
	let b64c = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
	let mut out = [0u8; 48];
	for i in 0..16 {
		let ia = b64c.iter().position(|x| *x == b[4*i]);
		let ib = b64c.iter().position(|x| *x == b[4*i+1]);
		let ic = b64c.iter().position(|x| *x == b[4*i+2]);
		let id = b64c.iter().position(|x| *x == b[4*i+3]);
		if ia.is_none() || ib.is_none() || ic.is_none() || id.is_none() { return Err("invalid base64 character"); }
		let group = (unsafe {ia.unwrap_unchecked()} as u32) << 18 | (unsafe {ib.unwrap_unchecked()} as u32) << 12 |
			(unsafe {ic.unwrap_unchecked()} as u32) << 6 | (unsafe {id.unwrap_unchecked()} as u32);
		out[3*i] = (group >> 16) as u8;
		out[3*i+1] = (group >> 8) as u8;
		out[3*i+2] = group as u8;
	}
	Ok(out)
}
