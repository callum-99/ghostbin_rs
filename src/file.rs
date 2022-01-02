use crate::{paste, languages, config};

use std::fs;

use xattr;

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste_id);
	if let Ok(paste_contents) = fs::read_to_string(&path) {
		let language = std::str::from_utf8(&xattr::get(path, "language").unwrap().unwrap()).unwrap().to_string();
		Ok(paste::Paste::new(paste_id, languages::get_language_by_name(language).unwrap(), false, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}

pub fn write_paste(paste: paste::Paste) -> Result<String, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste.id);
	if let Ok(_resp) = fs::write(&path, paste.code) {
		if let Ok(_) = xattr::set(path, "language", paste.language.name.as_bytes()) {
			Ok(paste.id)
		} else {
			Err("Can't set paste attributes".to_string())
		}
	} else {
		Err("Can't save paste".to_string())
	}
}