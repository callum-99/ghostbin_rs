use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

//use rocket::form::Form;

use crate::languages::Language;

#[derive(Deserialize, Serialize, Debug)]
pub struct Paste {
	pub id: String,
	pub language: Language,
	pub encrypted: bool,
	pub expiration: u64,
	pub created: u64,
	pub code: String,
}

impl Paste {
	pub fn simple(id: String, language: Language, encrypted: bool, code: String) -> Paste {
		Paste {
			id,
			language,
			encrypted,
			expiration: 0,
			created: SystemTime::now().duration_since(UNIX_EPOCH).expect("Error: negative time").as_millis() as u64,
			code,
		}
	}

	pub fn new(id: String, language: Language, encrypted: bool, expiration: u64, code: String) -> Paste {
		Paste {
			id,
			language,
			encrypted,
			expiration,
			created: SystemTime::now().duration_since(UNIX_EPOCH).expect("Error: negative time").as_millis() as u64,
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