use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

use rocket::form::Form;

use crate::languages::Language;

#[derive(Deserialize, Serialize, Debug)]
pub struct Paste {
	pub Id: String,
	pub Language: Language,
	pub Encrypted: bool,
	pub Expiration: Option<u64>,
	pub Created: u64,
	pub Code: String,
}

impl Paste {
	pub fn New(Id: String, Language: Language, Encrypted: bool, Code: String) -> Paste {
		Paste {
			Id,
			Language,
			Encrypted,
			Expiration: Some(0),
			Created: SystemTime::now().duration_since(UNIX_EPOCH).expect("Error: negative time").as_millis() as u64,
			Code: Code,
		}
	}
}

#[derive(Serialize, Debug, FromForm)]
pub struct UserInput {
	pub language: String,
	pub code: String
}