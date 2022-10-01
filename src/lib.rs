use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use roxmltree;

use std::error::Error;
use std::fs::File;
use std::io::Read;

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
    let mut file = File::open(config.bookmark_file_path).expect("Unaable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    // TODO: need a more robust replace in case people have weird bookmark titles or something
    let contents = contents.replace("<DT>","").replace("<p>","");

    let document = roxmltree::Document::parse(&contents)?;
    let root = Bookmark { title: "root".into(), content: Content::Folder(vec!()) };
    // dfs until we find an h* tag followed by a D* tag

    let xmlNode roxmltree::Node;

    fn recurse(xmlNode: roxmltree::Node, &mut wthNode: Boookmark) {
        if xmlNode.node_type != roxmltree::NodeType::Element {
            continue
        }
        if xmlNode.has_children() {
            for xmlChild in xmlNode.children() {
                recurse(
            }
        } else {
            wthNodej
        }
    }

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
