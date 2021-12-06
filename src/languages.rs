use serde::{Serialize, Deserialize};

//TODO: Load from json file not hardcode

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
	pub Name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
	pub CategoryName: String,
	pub Languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
	pub Categories: Vec<Category>,
}

pub fn Load() -> Languages {
	Languages {
		Categories: vec![
			Category {
				CategoryName: String::from("Text"),
				Languages: vec![
					Language {
						Name: String::from("Plain Text")
					},
					Language{
						Name: String::from("Markdown")
					},
				]
			},
			
			Category {
				CategoryName: String::from("C Family"),
				Languages: vec![
					Language {
						Name: String::from("C")
					},
					Language {
						Name: String::from("C++")
					},
					Language {
						Name: String::from("Objective-C")
					},
				]
			}
		]
	}
}