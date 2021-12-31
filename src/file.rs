use crate::{paste, languages, config};

use std::fs;

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	if let Ok(paste_contents) = fs::read_to_string(format!("{}/{}", config::Config::load().paste_path, paste_id)) {
		Ok(paste::Paste::new(paste_id, languages::Language{name: String::from("Plain Text")}, false, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}