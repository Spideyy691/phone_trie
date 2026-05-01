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