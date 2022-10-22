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
    TextInput(String),
    MultiTextInput(String, Vec<Dimension>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    title: String,
    content: Content,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dimension {
    title: String,
    points: Vec<String>,
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
        title: Option<String>,
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
            Content::TextInput(search_engine) => write!(f, "{} [{}]", self.title, search_engine),
            Content::MultiTextInput(template, dimensions) => {
                write!(f, "{} {} [{}]", self.title, template, dimensions.iter().map(|dimension| dimension.title.clone()).collect::<Vec<String>>().join("]["))
            }
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
        Action::Add{url, title} => {
            let mut root = load(&config.db)?;
            let title = match title {
                Some(title) => {
                    title
                },
                None => "New Bookmark".into()
            };
            let bookmark = if url.contains("%s") {
                Bookmark::new_search(title, url.clone())
            } else {
                Bookmark::new_link(title, url.clone())
            };
            root.add(bookmark)?;
            save(&config.db, &root)?;
            println!("Added {} to {}", &url, &config.db.to_string_lossy());
        },
        Action::Go => {
            let root = load(&config.db)?;
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
        Bookmark { title, content: Content::TextInput(content) }
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
            Content::TextInput(search_engine) => {
                let input = Input::<String>::new()
                    .interact_text()
                    .unwrap();
                let link = search(search_engine, &input);
                goto(link);
            },
            Content::MultiTextInput(template, dimensions) => {
                let separators = template.split("%s");
                let mut url = vec![];
                let mut fragments = dimensions.iter();
                for separator in separators {
                    url.push(separator);
                    let dimension = fragments.next().unwrap(); // TODO: handle cases when
                                                                     // they're not all the same
                                                                     // length
                    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt(&dimension.title)
                        .default(0)
                        .items(&dimension.points)
                        .interact()
                        .unwrap();
                    url.push(&dimension.points[selection]);
                }
                goto(url.join(""));
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

