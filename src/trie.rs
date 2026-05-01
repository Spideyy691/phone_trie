//! Trie implementation for phone numbers.
use crate::contact::Contact;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct Node {
    children: BTreeMap<char, Node>,
    contacts: Vec<Contact>,
}

/// Trie storing contacts by phone number.
#[derive(Debug, Default)]
pub struct PhoneTrie {
    root: Node,
}

impl PhoneTrie {
    /// Create an empty trie.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a contact using its phone number.
    pub fn insert(&mut self, contact: Contact) {
        let mut node = &mut self.root;
        for digit in contact.nb.chars() {
            node = node.children.entry(digit).or_default();
        }
        node.contacts.push(contact);
    }

    /// Build a trie from an iterator of contacts.
    pub fn from_contacts<I>(contacts: I) -> Self
    where
        I: IntoIterator<Item = Contact>,
    {
        let mut trie = Self::new();
        for contact in contacts {
            trie.insert(contact);
        }
        trie
    }

    /// Return all contacts whose number starts with `prefix`.
    pub fn search_prefix(&self, prefix: &str) -> Vec<&Contact> {
        let mut node = &self.root;
        for digit in prefix.chars() {
            match node.children.get(&digit) {
                Some(next) => node = next,
                None => return Vec::new(),
            }
        }

        let mut matches = Vec::new();
        collect_contacts(node, &mut matches);
        matches
    }

    /// Alias for `search_prefix`.
    pub fn search(&self, prefix: &str) -> Vec<&Contact> {
        self.search_prefix(prefix)
    }

    /// Export the trie as a PlantUML class diagram.
    pub fn to_plantuml(&self) -> String {
        let mut lines = vec!["@startuml".to_string(), "skinparam shadowing false".to_string()];
        let mut counter = 0usize;
        render_node(&self.root, &mut lines, &mut counter, None, None);
        lines.push("@enduml".to_string());
        lines.join("\n")
    }

    /// Alias for `to_plantuml`.
    pub fn export_plantuml(&self) -> String {
        self.to_plantuml()
    }
}

fn collect_contacts<'a>(node: &'a Node, out: &mut Vec<&'a Contact>) {
    out.extend(node.contacts.iter());
    for child in node.children.values() {
        collect_contacts(child, out);
    }
}

fn render_node(
    node: &Node,
    lines: &mut Vec<String>,
    counter: &mut usize,
    parent_id: Option<usize>,
    label: Option<char>,
) -> usize {
    let node_id = *counter;
    *counter += 1;

    let mut node_label = String::new();
    if let Some(digit) = label {
        node_label.push(digit);
    }
    if !node.contacts.is_empty() {
        if !node_label.is_empty() {
            node_label.push(' ');
        }
        let names: Vec<_> = node.contacts.iter().map(|contact| contact.name.as_str()).collect();
        node_label.push_str(&names.join(", "));
    }
    if node_label.is_empty() {
        node_label.push_str("root");
    }

    lines.push(format!("class node{} {{ {} }}", node_id, node_label));
    if let Some(parent_id) = parent_id {
        lines.push(format!("node{} --> node{}", parent_id, node_id));
    }

    for (digit, child) in &node.children {
        render_node(child, lines, counter, Some(node_id), Some(*digit));
    }

    node_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_search_returns_descendants() {
        let trie = PhoneTrie::from_contacts(vec![
            Contact { nb: "12".into(), name: "A".into() },
            Contact { nb: "123".into(), name: "B".into() },
            Contact { nb: "9".into(), name: "C".into() },
        ]);

        let names: Vec<_> = trie.search_prefix("12").into_iter().map(|contact| contact.name.as_str()).collect();
        assert_eq!(names, vec!["A", "B"]);
    }
}