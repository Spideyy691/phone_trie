use phone_trie::{loader, trie::PhoneTrie};

fn main() {
    match loader::load_contacts("data/04_common_parts.json") {
        Ok(contacts) => {
            let trie = PhoneTrie::from_contacts(contacts);
            println!("{}", trie.to_plantuml());
        }
        Err(err) => {
            eprintln!("failed to load contacts: {err}");
            std::process::exit(1);
        }
    }
}