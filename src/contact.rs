//! Entrée contact désérialisée depuis le fichier JSON.
use serde::Deserialize;

/// Un numéro de téléphone associé à un nom.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Contact {
    /// Numéro de téléphone (chiffres uniquement, sans préfixe international).
    pub nb: String,
    /// Nom d'affichage.
    pub name: String,
}