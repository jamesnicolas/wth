use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

pub mod parse;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fmt;
use parse::xml_string_to_bookmark;
use serde::{Deserialize, Serialize};
use clap::Subcommand;
use urlencoding::encode;
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


#[derive(Subcommand)]
pub enum Action {
    /// Imports a netscape bookmark file to path
    Import {
        path: PathBuf,
    },

    /// Adds a new bookmark to the db
    Add {
        url: String,
    },

    Go
}


pub struct Config {
    db: PathBuf,
    action: Action,
}

impl Config {
    pub fn new(db: PathBuf, action: Action) -> Self {
        Config { db, action }
    }
}

impl fmt::Display for Bookmark {
    // TODO: do something about long urls/wrapping
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            Content::Folder(_) => write!(f, "{} [...]", self.title),
            Content::Link(link) => write!(f, "{} [{}]", self.title, link),
            Content::Search(search_engine) => write!(f, "{} [{}]", self.title, search_engine),
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

    match config.action {
        Action::Import{path} => {
            let mut file = File::open(&path).expect("Unaable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read the file");
            let bookmark = xml_string_to_bookmark(contents)?;
            save(&config.db, &bookmark)?;
            println!("Imported {} to {}", &path.to_string_lossy(), &config.db.to_string_lossy());
        },
        Action::Add{url} => {
            let mut root = load(&config.db)?;
            let bookmark = Bookmark::new_link("New Bookmark".into(), url.clone());
            root.add(bookmark)?;
            save(&config.db, &root)?;
            println!("Added {} to {}", &url, &config.db.to_string_lossy());
        },
        Action::Go => {
            let mut root = load(&config.db)?;
            let google = Bookmark::new_search("Google".into(), "http://www.google.com/search?q=%s".into());
            root.add(google)?;
            root.prompt();
        },
    }

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

pub fn search(url: &str, query: &str) -> String {
    let url = url.to_string();
    let encoded_query = encode(query);
    url.replace("%s", &encoded_query)
}

impl Bookmark {
    pub fn new() -> Self {
        Bookmark { title: "".into(), content: Content::Folder(vec!()) }
    }

    pub fn new_link(title: String, content: String) -> Self {
        Bookmark { title, content: Content::Link(content) }
    }

    pub fn new_search(title: String, content: String) -> Self {
        Bookmark { title, content: Content::Search(content) }
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
            Content::Search(search_engine) => {
                let input = Input::<String>::new()
                    .interact_text()
                    .unwrap();
                let link = search(search_engine, &input);
                goto(link);
            },
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

