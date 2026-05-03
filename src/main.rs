//! Command-line entry point.
//!
//! Reads `data/04_common_parts.json`, builds the trie and writes the
//! corresponding PlantUML MindMap to `graph/04_common_parts.puml`.
use phone_trie::{loader, plantuml, trie::Trie};
const INPUT: &str = "data/04_common_parts.json";
const OUTPUT: &str = "graph/04_common_parts.puml";
fn main() -> std::io::Result<()> {
let contacts = loader::load_contacts(INPUT)?;
println!("Loaded {} contact(s) from {}", contacts.len(), INPUT);
let mut trie = Trie::new();
trie.insert_all(&contacts);
std::fs::create_dir_all("graph")?;
plantuml::write_mindmap(trie.root(), OUTPUT)?;
println!("Wrote PlantUML MindMap to {}", OUTPUT);
Ok(())
}