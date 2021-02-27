use std::{fs, path::PathBuf};

use clap::Clap;
use rustyline::{Editor, error::ReadlineError};
use directories_next::ProjectDirs;

use monkey_lang::lex;

#[derive(Clap)]
struct Opt {
    #[clap(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::parse();

    if opt.files.is_empty() {
        repl();
    } else {
        panic!("Running a file has not yet been implemented");
    }
}

fn repl() {
    let proj_dirs = ProjectDirs::from("rs", "Monkey Group", "Monkey Cli").unwrap();
    let data_dir = proj_dirs.data_dir();
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Failed to create data dir");
    }
    let history_file = data_dir.join("history.txt");

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(&history_file).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let lexed = lex(&line);
                println!("{:?}", lexed);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(&history_file).unwrap();
}
