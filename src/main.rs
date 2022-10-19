use std::process;
use clap::Parser;
use std::path::PathBuf;
use directories::ProjectDirs;
use wth::{Action, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom db file
    #[arg(short, long)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    action: Option<Action>,
}

fn main() {
    let cli = Cli::parse();

    let mut db_path: PathBuf;

    if let Some(proj_dirs) = ProjectDirs::from("com", "NicoLaser",  "wth") {
        db_path = proj_dirs.data_dir().to_path_buf();
        db_path.push("root.db")
    } else {
        panic!("Unable to get data dir")
    }

    let db = match cli.db {
        Some(custom_db) => custom_db,
        None => db_path,
    };

    let action = match cli.action {
        Some(some_action) => some_action,
        None => Action::Go,
    };

    let config = Config::new(db, action);

    if let Err(e) = wth::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}

