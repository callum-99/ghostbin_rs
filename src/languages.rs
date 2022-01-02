use serde::{Serialize, Deserialize};

//TODO: Load from json file not hardcode

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
	pub name: String,
	pub sname: String,
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
						name: String::from("Plain Text"),
						sname: String::from("plain_text")
					},
					Language{
						name: String::from("Markdown"),
						sname: String::from("markdown")
					},
				]
			},
			
			Category {
				category_name: String::from("C Family"),
				languages: vec![
					Language {
						name: String::from("C"),
						sname: String::from("c_cpp")
					},
					Language {
						name: String::from("C++"),
						sname: String::from("c_cpp")
					},
					Language {
						name: String::from("Objective-C"),
						sname: String::from("objectivec")
					},
				]
			},

			Category {
				category_name: String::from("Other"),
				languages: vec![
					Language {
						name: String::from("Apache Config"),
						sname: String::from("apache_conf")
					},
					Language {
						name: String::from("Apple Script"),
						sname: String::from("applescript")
					},
					Language {
						name: String::from("AsciiDoc"),
						sname: String::from("asciidoc")
					},
					Language {
						name: String::from("Assembly (x86)"),
						sname: String::from("assembly_x86")
					},
					Language {
						name: String::from("AutoHotKey"),
						sname: String::from("autohotkey")
					},
					Language {
						name: String::from("Batchfile"),
						sname: String::from("batchfile")
					},
					Language {
						name: String::from("Cobol"),
						sname: String::from("cobol")
					},
					Language {
						name: String::from("C#"),
						sname: String::from("csharp")
					},
					Language {
						name: String::from("CSS"),
						sname: String::from("css")
					},
					Language {
						name: String::from("Dart"),
						sname: String::from("dart")
					},
					Language {
						name: String::from("Django"),
						sname: String::from("django")
					},
					Language {
						name: String::from("Dot"),
						sname: String::from("dot")
					},
					Language {
						name: String::from("Forth"),
						sname: String::from("forth")
					},
					Language {
						name: String::from("GCode"),
						sname: String::from("gcode")
					},
					Language {
						name: String::from("Git Ignore"),
						sname: String::from("gitignore")
					},
					Language {
						name: String::from("GLSL"),
						sname: String::from("glsl")
					},
					Language {
						name: String::from("Go"),
						sname: String::from("golang")
					},
					Language {
						name: String::from("Handlebars"),
						sname: String::from("handlebars")
					},
					Language {
						name: String::from("Haskell"),
						sname: String::from("haskell")
					},
					Language {
						name: String::from("HJSON"),
						sname: String::from("hjson")
					},
					Language {
						name: String::from("HTML"),
						sname: String::from("html")
					},
					Language {
						name: String::from("INI"),
						sname: String::from("ini")
					},
					Language {
						name: String::from("Jave"),
						sname: String::from("java")
					},
					Language {
						name: String::from("JavaScript"),
						sname: String::from("javascript")
					},
					Language {
						name: String::from("JSON"),
						sname: String::from("json")
					},
					Language {
						name: String::from("Kotlin"),
						sname: String::from("kotlin")
					},
					Language {
						name: String::from("Latex"),
						sname: String::from("latex")
					},
					Language {
						name: String::from("Lisp"),
						sname: String::from("lisp")
					},
					Language {
						name: String::from("Lua"),
						sname: String::from("lua")
					},
					Language {
						name: String::from("Makefile"),
						sname: String::from("makefile")
					},
					Language {
						name: String::from("Matlab"),
						sname: String::from("matlab")
					},
					Language {
						name: String::from("MYSQL"),
						sname: String::from("mysql")
					},
					Language {
						name: String::from("Nginx Config"),
						sname: String::from("nginx")
					},
					Language {
						name: String::from("Nim"),
						sname: String::from("nim")
					},
					Language {
						name: String::from("Pascal"),
						sname: String::from("pascal")
					},
					Language {
						name: String::from("Perl"),
						sname: String::from("perl")
					},
					Language {
						name: String::from("PHP"),
						sname: String::from("php")
					},
					Language {
						name: String::from("Python"),
						sname: String::from("python")
					},
					Language {
						name: String::from("Ruby"),
						sname: String::from("ruby")
					},
					Language {
						name: String::from("Scala"),
						sname: String::from("scala")
					},
					Language {
						name: String::from("SCSS"),
						sname: String::from("scss")
					},
					Language {
						name: String::from("Shell Script"),
						sname: String::from("sh")
					},
					Language {
						name: String::from("SQL"),
						sname: String::from("sql")
					},
					Language {
						name: String::from("SQL Server"),
						sname: String::from("sql_server")
					},
					Language {
						name: String::from("Swift"),
						sname: String::from("Swift")
					},
					Language {
						name: String::from("TOML"),
						sname: String::from("toml")
					},
					Language {
						name: String::from("TypeScript"),
						sname: String::from("typescript")
					},
					Language {
						name: String::from("VBScript"),
						sname: String::from("vbscript")
					},
					Language {
						name: String::from("Verilog"),
						sname: String::from("verilog")
					},
					Language {
						name: String::from("VHDL"),
						sname: String::from("vhdl")
					},
					Language {
						name: String::from("XML"),
						sname: String::from("xml")
					},
					Language {
						name: String::from("YAML"),
						sname: String::from("yaml")
					},
					
				]
			}
		]
	}
}

pub fn get_language_by_name(name: String) -> Result<Language, String> {
	for category in load().categories {
		for language in category.languages {
			if language.name == name {
				return Ok(language);
			}
		}
	}

	return Err("Cant find the language".to_string());
}

pub fn get_language_by_sname(sname: String) -> Result<Language, String> {
	for category in load().categories {
		for language in category.languages {
			if language.sname == sname {
				return Ok(language);
			}
		}
	}

	return Err("Cant find the language".to_string());
}