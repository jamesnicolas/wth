use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub enum Content {
    Folder(Vec<Bookmark>),
    Link(String),
    Search(String),
}

#[derive(Debug)]
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
    let mut root = Bookmark { title: "root".into(), content: Content::Folder(vec!()) };

    fn traverse(xml_node: &roxmltree::Node, parent_tag: &str, parent_bookmark: &mut Bookmark) {
        let mut tag_name: Option<&str> = None;
        match xml_node.node_type() {
            roxmltree::NodeType::Element =>  {
                tag_name = Some(xml_node.tag_name().name());
                if let Some(attribute) = xml_node.attribute("HREF") {
                    // since we're in an A tag, we can get the first child which should be text,
                    // which should be the title
                    let title: &str;
                    match xml_node.children().next() {
                        Some(text) => title = match text.node_type() {
                            roxmltree::NodeType::Text => text.text().expect("Text.text() should always be Some(&str)"),
                            _ => attribute,
                        },
                        None => title = attribute,
                    };

                    let bookmark = Bookmark { title: title.into(),  content: Content::Link(attribute.to_string()) };

                    if let Content::Folder(content) = &mut parent_bookmark.content {
                        content.push(bookmark);
                    }
                }
            },
            roxmltree::NodeType::Root => (),
            roxmltree::NodeType::Text => (),
            roxmltree::NodeType::Comment => return,
            roxmltree::NodeType::PI => return,
        }
        for xml_child in xml_node.children() {
            traverse(&xml_child, tag_name.or(Some("")).unwrap(), parent_bookmark);
        }
    }


    traverse(&document.root(), "", &mut root);

    prompt(&root);

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

pub fn prompt(root: &Bookmark) {
    let items = root.items();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bookmark")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    println!("You picked {}", items[selection]);
}
