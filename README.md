# phone_trie - partie C

Ce branche contient la partie C du projet : une interface CLI pour le trie de numéros.

## Usage

```bash
cargo run -- --input data/04_common_parts.json --search 04
cargo run -- --input data/04_common_parts.json --diagram
cargo run -- --input data/04_common_parts.json --export trie.puml
```

## Options

- `--input PATH` : fichier JSON de contacts (par défaut `data/04_common_parts.json`).
- `--search PREFIX` : recherche les contacts dont le numéro commence par le préfixe.
- `--diagram` : affiche le diagramme PlantUML.
- `--export PATH` : exporte le diagramme PlantUML dans un fichier.

## Tests

La branche ajoute des tests pour :
- la lecture du dataset JSON,
- la recherche de préfixe dans le trie,
- l’export PlantUML du trie.
