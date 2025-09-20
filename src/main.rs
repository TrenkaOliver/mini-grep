use std::{env, error::Error, fs, process};

use mini_grep::*;

struct Config {
    query: String,
    file_path: String,
    igonore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or_else(|| "no argument found for query!")?;
        let file_path = args.next().ok_or_else(|| "no argument found for file path!")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query: query, file_path: file_path, igonore_case: ignore_case })
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.igonore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}