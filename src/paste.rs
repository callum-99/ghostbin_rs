use std::time::{SystemTime};

use serde::{Serialize, Deserialize};

//use rocket::form::Form;

use crate::languages::Language;

#[derive(Deserialize, Serialize, Debug)]
pub struct Paste {
	pub id: String,
	pub language: Language,
	pub encrypted: bool,
	pub expired: bool,
	pub expiration: u64,
	pub code: String,
}

impl Paste {
	pub fn simple(id: String, language: Language, encrypted: bool, expired: bool, code: String) -> Paste {
		Paste {
			id,
			language,
			encrypted,
			expired,
			expiration: 0,
			code,
		}
	}

	pub fn new(id: String, language: Language, encrypted: bool, expiration: u64, code: String) -> Paste {
		Paste {
			id,
			language,
			encrypted,
			expired: false,
			expiration: match expiration == 0 {
				true => 0,
				false => SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("get millis error").as_millis() as u64 + expiration
			},
			code,
		}
	}
}

#[derive(Serialize, Debug, FromForm)]
pub struct UserInput {
	pub language: String,
	pub code: String,
	pub encryption: Option<String>,
	pub expiration: u64
}