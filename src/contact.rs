//! Contact entry deserialised from the input JSON file.
use serde::Deserialize;

/// A phone number associated with a name.
#[derive(Debug, Deserialize)]
pub struct Contact {
    /// Phone number (digits only, no international prefix).
    pub nb: String,
    /// Display name.
    pub name: String,
}