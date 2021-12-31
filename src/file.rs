use crate::{paste, languages, config};

use std::fs;

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	if let Ok(paste_contents) = fs::read_to_string(format!("{}/{}", config::Config::load().paste_path, paste_id)) {
		Ok(paste::Paste::new(paste_id, languages::Language{name: String::from("Plain Text")}, false, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}

pub fn write_paste(paste: paste::Paste) -> Result<String, String> {
	if let Ok(resp) = fs::write(format!("{}/{}", config::Config::load().paste_path, paste.id), paste.code) {
		Ok(paste.id)
	} else {
		Err("Can't save paste".to_string())
	}
}