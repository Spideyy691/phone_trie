//! From-scratch prefix tree (trie) for storing phone numbers.

/// Number of possible digits at each position (0..=9).
pub const ALPHABET_SIZE: usize = 10;

/// One node of the trie.
#[derive(Debug, Default)]
pub struct TrieNode {
    /// Children indexed by digit value.
    pub children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
    /// Names of contacts whose number ends at this node.
    pub names: Vec<String>,
}

/// A trie of phone numbers.
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Create an empty trie.
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrow the root node (used by the PlantUML exporter).
    pub fn root(&self) -> &TrieNode {
        &self.root
    }

    /// Insert a contact into the trie.
    ///
    /// Walks down the tree digit by digit, creating missing nodes along the way,
    /// then appends the contact name to the terminal node.
    pub fn insert(&mut self, contact: &crate::contact::Contact) {
        let mut current = &mut self.root;
        for ch in contact.nb.chars() {
            let digit = ch.to_digit(10)
                .expect("phone number must contain only digits") as usize;
            current = current.children[digit]
                .get_or_insert_with(|| Box::new(TrieNode::default()));
        }
        current.names.push(contact.name.clone());
    }

    /// Insert all contacts from a slice.
    pub fn insert_all(&mut self, contacts: &[crate::contact::Contact]) {
        for c in contacts {
            self.insert(c);
        }
    }
}