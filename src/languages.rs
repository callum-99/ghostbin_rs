use serde::{Serialize, Deserialize};

//TODO: Load from json file not hardcode

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
	pub category_name: String,
	pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
	pub categories: Vec<Category>,
}

pub fn load() -> Languages {
	Languages {
		categories: vec![
			Category {
				category_name: String::from("Text"),
				languages: vec![
					Language {
						name: String::from("Plain Text")
					},
					Language{
						name: String::from("Markdown")
					},
				]
			},
			
			Category {
				category_name: String::from("C Family"),
				languages: vec![
					Language {
						name: String::from("C")
					},
					Language {
						name: String::from("C++")
					},
					Language {
						name: String::from("Objective-C")
					},
				]
			}
		]
	}
}
