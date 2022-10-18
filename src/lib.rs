use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub mod parse;

use std::error::Error;
use std::path::{Path, PathBuf};
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
    db: PathBuf,
    import: Option<PathBuf>,
}

impl Config {
    pub fn new(db: PathBuf, import: Option<PathBuf>) -> Self {
        Config { db, import }
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

pub fn save(db: &Path, bookmark: &Bookmark) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(db);
    let prefix = path.parent().ok_or("invalid path")?;
    std::fs::create_dir_all(prefix)?;
    let mut file = File::create(db).expect("Unable to open the file");
    file.write_all(ron::to_string(bookmark).expect("Unable to convert bookmark").as_bytes()).expect("Unable to write to file");
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.import {
        Some(import) => {
            let mut file = File::open(&import).expect("Unaable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read the file");
            let bookmark = xml_string_to_bookmark(contents)?;
            save(&config.db, &bookmark)?;
        },
        None => ()
    }
    let mut root = load(&config.db)?;

    let add_test = Bookmark::new_link("brand new bookmark!".into(), "https://jamesnicolas.com".into());

    root.add(add_test)?;

    root.prompt();

    Ok(())
}

pub fn load(db: &Path) -> Result<Bookmark, Box<dyn Error>> {
    let mut file = File::open(db)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let bookmark = ron::from_str(&contents)?;
    return Ok(bookmark)
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

    pub fn add(&mut self, other: Self) -> Result<(), Box<dyn Error>> {
        match &mut self.content {
            Content::Folder(folder) => {
                folder.push(other);
            },
            _ => return Err("Cannot append to non-folder type".into()),
        };
        Ok(())
    }
}

