use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub enum Content {
    Folder(Vec<Bookmark>),
    Link(String),
    Search(String),
}

pub struct Bookmark {
    title: String,
    content: Content,
}

pub struct Config {
    bookmark_file_path: String,
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

        Ok(Config { bookmark_file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.bookmark_file_path)?;
    let reader = BufReader::new(file);

    let raw: Value = serde_json::from_reader(reader)?;

    println!("checksum is {}", raw["checksum"]);
    Ok(())
}


impl Bookmark {
    pub fn items(&self) -> Vec<String> {
        match &self.content {
            Content::Folder(folder) => {
                folder.iter().map(|item| item.title.to_string()).collect::<Vec<String>>()
            },
            Content::Link(link) => vec!(link.to_string()),
            Content::Search(search) => vec!(search.to_string()),
        }
    }

}

pub fn prompt() {
    let root = Bookmark{
        title: "root".to_string(),
        content: Content::Folder(vec!(Bookmark{title: "google.com".to_string(), content: Content::Link("google.com".to_string())})),
    };

    let items = root.items();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bookmark")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    println!("You picked {}", items[selection]);
}
