use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub enum Content {
    Folder(Vec<Bookmark>),
    Link(String),
    Search(String),
}

pub struct Bookmark {
    title: String,
    content: Content,
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
