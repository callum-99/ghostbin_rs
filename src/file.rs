use crate::{paste, languages, config};

use std::fs;

use xattr;

fn get_xattr(xattr: &str, path: &str) -> String {
	let mut xattrs = xattr::list(path).unwrap().peekable();
	if xattrs.peek().is_none() {
    return "".to_string()
	}

	for xattr in xattrs {
		println!("{:?}", xattr)
	}

	"".to_string()
}

pub fn get_paste(paste_id: String) -> Result<paste::Paste, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste_id);
	if let Ok(paste_contents) = fs::read_to_string(&path) {
		// let language = get_xattr("language", &path);
		let language = std::str::from_utf8(&xattr::get(path, "language").unwrap().unwrap()).unwrap().to_string();
		Ok(paste::Paste::new(paste_id, languages::Language{name: language}, false, paste_contents))
	} else {
		Err("Paste not found".to_string())
	}
}

pub fn write_paste(paste: paste::Paste) -> Result<String, String> {
	let path = format!("{}/{}", config::Config::load().paste_path, paste.id);
	if let Ok(_resp) = fs::write(&path, paste.code) {
		xattr::set(path, "language", paste.language.name.as_bytes());
		Ok(paste.id)
	} else {
		Err("Can't save paste".to_string())
	}
}