use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub mod parse;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fmt;
use parse::xml_string_to_bookmark;
use serde::{Deserialize, Serialize};
use open;
use ron;

#[derive(Debug, Deserialize, Serialize)]
pub enum Content {
    Folder(Vec<Bookmark>),
    Link(String),
    Search(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    title: String,
    content: Content,
}

pub struct Config {
    bookmark_file_path: String,
    save_file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();
        let bookmark_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No bookmark file specified"),
        };

        let save_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No save file specified"),
        };

        Ok(Config { bookmark_file_path, save_file_path })
    }
}

impl fmt::Display for Bookmark {
    // TODO: do something about long urls/wrapping
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            Content::Folder(_) => write!(f, "{} [...]", self.title),
            Content::Link(link) => write!(f, "{} [{}]", self.title, link),
            Content::Search(_) => write!(f, "{} [not implemented yet!]", self.title),
        }
    }
}

pub fn save(save_file_path: &str, bookmark: &Bookmark) {
    let mut file = File::create(save_file_path).expect("Unable to open the file");
    file.write_all(ron::to_string(bookmark).expect("Unable to convert bookmark").as_bytes()).expect("Unable to write to file");
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.bookmark_file_path).expect("Unaable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let root = xml_string_to_bookmark(contents)?;

    save(&config.save_file_path, &root);

    root.prompt();

    Ok(())
}

pub fn goto(arg: impl fmt::Display + std::convert::AsRef<std::ffi::OsStr>) {
    open::that(arg).unwrap();
}


impl Bookmark {
    pub fn new() -> Self {
        Bookmark { title: "".into(), content: Content::Folder(vec!()) }
    }

    pub fn new_link(title: String, content: String) -> Self {
        Bookmark { title, content: Content::Link(content) }
    }
    pub fn prompt(&self) {
        match &self.content {
            Content::Folder(folder) => {
                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select bookmark")
                    .default(0)
                    .items(folder)
                    .interact()
                    .unwrap();
                let bookmark = &folder[selection];
                bookmark.prompt();
            },
            Content::Link(link) => goto(link),
            Content::Search(search) => goto(search),
        };
    }

}

