//! Integration tests. Each team member adds their tests in their own block.
use phone_trie::{loader, trie::PhoneTrie};

// ===== Membre A : data & I/O =====
#[test]
fn loader_reads_test_dataset() {
    let contacts = loader::load_contacts("data/04_common_parts.json")
        .expect("test data file should be readable");
    assert_eq!(contacts.len(), 5);
    assert_eq!(contacts[0].name, "Alice");
    assert_eq!(contacts[0].nb, "0412578440");
}

#[test]
fn loader_fails_on_missing_file() {
    let result = loader::load_contacts("data/does_not_exist.json");
    assert!(result.is_err());
}

#[test]
fn trie_search_prefix_returns_descendants() {
    let trie = PhoneTrie::from_contacts(vec![
        phone_trie::contact::Contact { nb: "12".into(), name: "A".into() },
        phone_trie::contact::Contact { nb: "123".into(), name: "B".into() },
        phone_trie::contact::Contact { nb: "9".into(), name: "C".into() },
    ]);

    let names: Vec<_> = trie
        .search_prefix("12")
        .into_iter()
        .map(|contact| contact.name.as_str())
        .collect();
    assert_eq!(names, vec!["A", "B"]);
}

#[test]
fn trie_exports_plantuml() {
    let trie = PhoneTrie::from_contacts(vec![
        phone_trie::contact::Contact { nb: "12".into(), name: "Alice".into() },
        phone_trie::contact::Contact { nb: "123".into(), name: "Bob".into() },
    ]);

    let diagram = trie.to_plantuml();
    assert!(diagram.contains("@startuml"));
    assert!(diagram.contains("@enduml"));
    assert!(diagram.contains("Alice"));
    assert!(diagram.contains("Bob"));
}
