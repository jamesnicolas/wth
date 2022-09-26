use dialoguer::{theme::ColorfulTheme, FuzzySelect};

enum Bookmark<'a> {
    Folder(String, &'a Vec<String>),
    Link(String),
    Search(String),
}

fn main() {
    let folder = vec!(String::from("google.com"), String::from("github.com"));
    let default = Bookmark::Folder("default".to_string(), &folder);

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bookmark")
        .default(0)
        .items(if let Bookmark::Folder(_, list) = default{list} else {&[]})
        .interact()
        .unwrap();

    println!("You picked {}", folder[selection]);

}
