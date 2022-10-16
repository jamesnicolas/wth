use std::process;
use std::path::PathBuf;
use directories::ProjectDirs;
use wth::Config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom db file
    #[arg(short, long)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Import {
        /// lists test values
        path: PathBuf,
    },
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

    let import = &cli.command.map(|command| {
        match command {
            Commands::Import{path} => path
        }
    });
    let config = Config::new(db.into(), import.to_owned());

    if let Err(e) = wth::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}

