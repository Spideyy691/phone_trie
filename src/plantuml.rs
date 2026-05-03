//! Export the trie as a PlantUML MindMap.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::trie::TrieNode;

/// Convert a trie into PlantUML MindMap source code.
///
/// The root node itself is not rendered â€” the first emitted level corresponds
/// to digits that are direct children of the root.
pub fn trie_to_mindmap(root: &TrieNode) -> String {
    let mut out = String::from("@startmindmap\n");
    for (digit, child) in root.children.iter().enumerate() {
        if let Some(node) = child {
            write_node(&mut out, node, digit, 1);
        }
    }
    out.push_str("@endmindmap\n");
    out
}

fn write_node(out: &mut String, node: &TrieNode, digit: usize, depth: usize) {
    out.push_str(&"*".repeat(depth));
    out.push(' ');
    out.push(char::from_digit(digit as u32, 10).unwrap());
    out.push('\n');

    for name in &node.names {
        out.push_str(&"*".repeat(depth + 1));
        out.push(' ');
        out.push_str(name);
        out.push('\n');
    }

    for (d, child) in node.children.iter().enumerate() {
        if let Some(c) = child {
            write_node(out, c, d, depth + 1);
        }
    }
}

/// Convenience wrapper: generate the MindMap and write it to `path`.
pub fn write_mindmap<P: AsRef<Path>>(root: &TrieNode, path: P) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writer.write_all(trie_to_mindmap(root).as_bytes())?;
    Ok(())
}