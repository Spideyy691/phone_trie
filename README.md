# phone_trie

Projet Rust pour stocker des numéros de téléphone dans un trie et exporter une représentation PlantUML.

## Installation

```bash
cargo build --release
```

## Exécution

```bash
cargo run -- --input data/04_common_parts.json
```

## Options

- `--input PATH` : fichier JSON de contacts (défaut `data/04_common_parts.json`).
- `--search PREFIX` : recherche les contacts dont le numéro commence par `PREFIX`.
- `--export PATH` : exporte le diagramme PlantUML dans le fichier indiqué.
- `--diagram` : affiche le diagramme PlantUML sur la sortie standard.
- `--help` : affiche cette aide.

## Exemple

```bash
cargo run -- --search 04
cargo run -- --export trie.puml
```
