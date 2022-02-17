use crate::{paste, languages, config};

use std::fs;

use xattr;

use std::time::SystemTime;

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste_id);
	if let Ok(paste_contents) = fs::read_to_string(&path) {
		let language = std::str::from_utf8(&xattr::get(&path, "user.language").unwrap().unwrap()).unwrap().to_string();
		let encrypted: bool = std::str::from_utf8(&xattr::get(&path, "user.encrypted").unwrap().unwrap()).unwrap().to_string().parse().unwrap();
		let expiration = str::parse::<u64>(std::str::from_utf8(&xattr::get(&path, "user.expiration").unwrap().unwrap()).unwrap()).unwrap();

		// Check if post has expired if so delete it
		let mut expired = false;

		if expiration != 0 {
			let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("get millis error");

			if now.as_millis() as u64 > expiration {
				expired = true;
		
				fs::remove_file(&path);
				return Err("Paste expired".to_string());
			}
		}
			
		Ok(paste::Paste::simple(paste_id, languages::get_language_by_sname(language).unwrap(), encrypted, expired, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}

pub fn set_xattr(path: &String, language: &String, encrypted: &bool, expiration: u64) -> Result<(), String> {
	match xattr::set(path, "user.language", language.as_bytes()) {
		Ok(_) => {
			match xattr::set(path, "user.encrypted", encrypted.to_string().as_bytes()) {
				Ok(_) => {
					match xattr::set(path, "user.expiration", expiration.to_string().as_bytes()) {
						Ok(_) => return Ok(()),
						Err(e) => {
							return Err(format!("Can't set expiration xattrs: {}", e))
						}
					}
				},
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
		Ok(_) =>  match set_xattr(&path, &paste.language.sname, &paste.encrypted, paste.expiration) {
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