use std::env;
use std::fs;
use phone_trie::{loader, trie::PhoneTrie};

struct Config {
    input: String,
    search: Option<String>,
    export: Option<String>,
    diagram: bool,
}

impl Config {
    fn from_args() -> Self {
        let mut input = String::from("data/04_common_parts.json");
        let mut search = None;
        let mut export = None;
        let mut diagram = false;

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    Self::print_usage();
                    std::process::exit(0);
                }
                "--input" => {
                    input = args.next().unwrap_or_else(|| {
                        eprintln!("Expected path after --input");
                        std::process::exit(1);
                    });
                }
                "--search" => {
                    search = Some(args.next().unwrap_or_else(|| {
                        eprintln!("Expected prefix after --search");
                        std::process::exit(1);
                    }));
                }
                "--export" => {
                    export = Some(args.next().unwrap_or_else(|| {
                        eprintln!("Expected path after --export");
                        std::process::exit(1);
                    }));
                }
                "--diagram" => {
                    diagram = true;
                }
                unknown => {
                    eprintln!("Unknown option: {unknown}");
                    Self::print_usage();
                    std::process::exit(1);
                }
            }
        }

        Self {
            input,
            search,
            export,
            diagram,
        }
    }

    fn print_usage() {
        eprintln!("Usage: phone_trie [--input PATH] [--search PREFIX] [--export PATH] [--diagram]");
        eprintln!("    --input PATH   JSON contacts file (default data/04_common_parts.json)");
        eprintln!("    --search PREFIX   search for contacts matching the prefix");
        eprintln!("    --export PATH   save PlantUML diagram to PATH");
        eprintln!("    --diagram   print the PlantUML diagram to stdout");
    }
}

fn main() {
    let config = Config::from_args();

    let contacts = match loader::load_contacts(&config.input) {
        Ok(contacts) => contacts,
        Err(err) => {
            eprintln!("failed to load contacts: {err}");
            std::process::exit(1);
        }
    };

    let trie = PhoneTrie::from_contacts(contacts);

    if let Some(ref prefix) = config.search {
        let matches = trie.search(prefix);
        if matches.is_empty() {
            println!("No contacts found for prefix '{prefix}'");
        } else {
            for contact in matches {
                println!("{} -> {}", contact.nb, contact.name);
            }
        }
    }

    if config.diagram || config.export.is_some() {
        let diagram = trie.to_plantuml();
        if let Some(ref path) = config.export {
            if let Err(err) = fs::write(path, diagram) {
                eprintln!("failed to export diagram: {err}");
                std::process::exit(1);
            }
            println!("diagram written to {path}");
        } else {
            println!("{diagram}");
        }
    }

    if config.search.is_none() && !config.diagram && config.export.is_none() {
        println!("{}", trie.to_plantuml());
    }
}
