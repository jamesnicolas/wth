use dialoguer::{theme::ColorfulTheme, FuzzySelect};

enum Content<'a> {
    Folder(&'a [Bookmark<'a>]),
    Link(&'a str),
    Search(&'a str),
}

struct Bookmark<'a> {
    title: &'a str,
    content: Content<'a>
}

impl<'a> Bookmark<'a> {
    fn items(&'a self) -> Vec<String> {
        match self.content {
            Content::Folder(folder) => {
                let content = folder.iter().map(|item| item.title.to_string()).collect::<Vec<String>>();
                content
            },
            Content::Link(link) => vec!(link.to_string()),
            Content::Search(search) => vec!(search.to_string()),
        }
    }

}

fn main() {
    let root = Bookmark{
        title: "root",
        content: Content::Folder(&[Bookmark{title: "google.com", content: Content::Link("google.com")}]),
    };

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bookmark")
        .default(0)
        .items(&root.items())
        .interact()
        .unwrap();

    println!("You picked {}", selection);

}
