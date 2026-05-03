//! `phone_trie` — store phone numbers in a trie and export them as PlantUML.
//!
//! # Example
//!
//! ```no_run
//! use phone_trie::{loader, plantuml, trie::Trie};
//!
//! let contacts = loader::load_contacts("data/04_common_parts.json").unwrap();
//! let mut trie = Trie::new();
//! trie.insert_all(&contacts);
//! plantuml::write_mindmap(trie.root(), "graph/04_common_parts.puml").unwrap();
//! ```
#![forbid(unsafe_code)]
pub mod contact;
pub mod loader;
pub mod plantuml;
pub mod trie;