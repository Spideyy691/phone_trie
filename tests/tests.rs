//! Integration tests. Each team member adds their tests in their own block.
use phone_trie::loader;

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
// ===== C : plantuml =====
use phone_trie::{plantuml, trie::Trie, contact::Contact};

#[test]
fn mindmap_has_correct_markers() {
    let trie = Trie::new();
    let out = plantuml::trie_to_mindmap(trie.root());
    assert!(out.starts_with("@startmindmap\n"));
    assert!(out.trim_end().ends_with("@endmindmap"));
}

#[test]
fn mindmap_indentation_matches_depth() {
    let mut trie = Trie::new();
    trie.insert(&Contact { nb: "112".into(), name: "Urgences".into() });
    let out = plantuml::trie_to_mindmap(trie.root());
    assert!(out.contains("* 1\n"));
    assert!(out.contains("** 1\n"));
    assert!(out.contains("*** 2\n"));
    assert!(out.contains("**** Urgences\n"));
}
// ===== Adam F: end-to-end =====

#[test]
fn full_pipeline() {
    let contacts = loader::load_contacts("data/04_common_parts.json").unwrap();
    let mut trie = Trie::new();
    trie.insert_all(&contacts);
    let mindmap = plantuml::trie_to_mindmap(trie.root());

    // Each contact name must appear in the diagram.
    for c in &contacts {
        assert!(mindmap.contains(&c.name),
            "expected name '{}' in mindmap output", c.name);
    }
    // The shared prefix "0412" produces these specific lines.
    assert!(mindmap.contains("* 0\n"));
    assert!(mindmap.contains("**** 2\n"));
}