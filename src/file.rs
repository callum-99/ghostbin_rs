use crate::{paste, languages, config};

use std::fs;

use xattr;

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste_id);
	if let Ok(paste_contents) = fs::read_to_string(&path) {
		let language = std::str::from_utf8(&xattr::get(path, "language").unwrap().unwrap()).unwrap().to_string();
		Ok(paste::Paste::simple(paste_id, languages::get_language_by_sname(language).unwrap(), false, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}

pub fn set_xattr(path: &String, language: &String, encrypted: bool) -> Result<(), String> {
	match xattr::set(path, "language", language.as_bytes()) {
		Ok(_) => {
			match xattr::set(path, "encrypted", &[encrypted as u8]) {
				Ok(_) => return Ok(()),
				Err(e) => {
					return Err(format!("Can't set encrypted xattrs: {}", e))
				}
			}
		},
		Err(e) => {
			return Err(format!("Can't set language xattrs: {}", e))
		}
	}
}

pub fn write(paste: paste::Paste) -> Result<(), String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste.id);
	
	match fs::write(&path, paste.code.as_bytes()) {
		Ok(_) =>  match set_xattr(&path, &paste.language.sname, paste.encrypted) {
			Ok(_) => return Ok(()),
			Err(e) => { 
				println!("Error: {}", e);
				Err(format!("Failed to set xattrs paste: {:?} to {}", &paste, path))
			}
		},

		Err(e) => {
			println!("Error: {}", e);
			return Err(format!("Failed to write paste: {:?} to {}", &paste, path));
		}
	}
}