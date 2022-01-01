#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template};
use rocket::fs::{FileServer, relative};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::Status;

use uuid::Uuid;

use std::env;

mod paste;
mod languages;
mod file;
mod config;

#[get("/")]
fn index() -> Template {
	let languages = languages::load();
	Template::render("index", &languages)
}

#[post("/new", data = "<user_input>")]
fn new_paste(user_input: Form<paste::UserInput>) -> Result<Redirect, Status> {
	let id = format!("{}", Uuid::new_v4().to_simple());
	let paste = paste::Paste::new((&id).to_string(), languages::Language{name: String::from(&user_input.language)}, false, String::from(&user_input.code));
	if let Ok(_) = file::write_paste(paste) {
		Ok(Redirect::to(format!("/paste/{}", id)))
	} else {
		Err(Status::InternalServerError)
	}
}

#[get("/paste/<paste_id>")]
fn get_paste(paste_id: String) -> Result<Template, Status> {
	if let Ok(paste) = file::get_paste(paste_id) {
		Ok(Template::render("paste", &paste))
	} else {
		Err(Status::NotFound)
	}
}

#[get("/paste/<paste_id>/raw")]
fn get_paste_raw(paste_id: String) -> Result<String, Status> {
	if let Ok(paste) = file::get_paste(paste_id) {
		Ok(paste.code)
	} else {
		Err(Status::NotFound)
	}
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index, new_paste, get_paste, get_paste_raw])
		.mount("/static", FileServer::from(relative!("static"))) 
		.attach(Template::fairing())
}
