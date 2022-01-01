#[derive(Debug)]
pub struct Config {
	pub paste_path: String,
	pub admin_password: String,
	pub log_verbosity: u8,
}

impl Config {
	// TODO: read from config file supplied in args
	pub fn load() -> Self {
		Self {
			paste_path: "/tmp/pastes".to_string(),
			admin_password: "test_password".to_string(),
			log_verbosity: 5
		}
	}
}