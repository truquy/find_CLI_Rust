use std::error::Error;
use std::fs;
use regex::Regex;
use std::env;
use std::process;

pub fn run(cli: CLI) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(cli.file)?;

    let re = Regex::new(&format!(".*{}.*", cli.text)).unwrap();

    for (number, line) in content.lines().enumerate() {
        if re.is_match(line) {
            println!("\nLine {}: {}\n", number + 1, line);
            return Ok(());
        }
    }

    eprintln!("\nText not found in file\n");
    Ok(())
}

pub struct CLI {
    pub text: String,
    pub file: String,
}

impl CLI {
    pub fn new(cli_args: &[String]) -> Result<CLI, &str> {
        if cli_args.len() < 3 {
            return Err("\nFormat as : cargo run find_text filename.extension \n");
        }

        let text = cli_args[1].clone();
        let file = cli_args[2].clone();

        Ok(CLI { text, file })
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let cli = CLI::new(&cli_args).unwrap_or_else(|err| {
        eprintln!("\nError parsing arguments: {}\n", err);
        process::exit(1);
    });

    if let Err(e) = run(cli) {
        eprintln!("\nApplication error: {}\n ", e);
        process::exit(1);
    }
}