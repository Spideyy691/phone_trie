//! Lecture de la liste de contacts depuis un fichier JSON.
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::contact::Contact;

/// Désérialise le fichier JSON
pub fn load_contacts<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Contact>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let contacts = serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(contacts)
}