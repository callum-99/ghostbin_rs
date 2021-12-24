#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, handlebars};
use rocket::fs::{FileServer, relative};
use rocket::form::Form;
use rocket::response::Redirect;

mod paste;

mod languages;

#[get("/")]
fn index() -> Template {
	let context = languages::load();
	Template::render("index", &context)
}

#[post("/new", data = "<user_input>")]
fn new_paste(user_input: Form<paste::UserInput>) -> Redirect {
	println!("Language: '{}'\nCode: '{}'", user_input.language, user_input.code);
	Redirect::to(format!("/paste/{}", 1))
}

#[get("/paste/<paste_id>")]
fn get_paste(paste_id: String) -> Template {
	let context = paste::Paste::new(paste_id, languages::Language{name: String::from("Plain Text")}, false, String::from("Test code"));

	Template::render("paste", &context)
}

#[get("/paste/<paste_id>/raw")]
fn get_paste_raw(paste_id: String) -> String {
	let paste = paste::Paste::new(paste_id, languages::Language{name: String::from("Plain Text")}, false, String::from("Test code"));

	paste.code
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index, new_paste, get_paste, get_paste_raw])
		.mount("/static", FileServer::from(relative!("static"))) 
		.attach(Template::fairing())
}
