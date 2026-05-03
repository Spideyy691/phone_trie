# phone_trie — Gestionnaire de numéros de téléphone

Stocke des numéros de téléphone dans un trie (prefix tree) implémenté
**from scratch** et exporte la structure au format **PlantUML MindMap**.

## Auteurs

- Mathieu — chargement JSON & struct Contact (contact.rs, loader.rs)
- Antoine — trie from scratch (trie.rs)
- Alexis — export PlantUML (plantuml.rs)
- Adam  — CLI, intégration & doc (lib.rs, main.rs, README)

## Prérequis

- Rust stable (>= 1.75) et Cargo.


## Lancement

```bash
# Compile et exécute en mode release.
# Lit data/04_common_parts.json, écrit graph/04_common_parts.puml
cargo run --release
```

## Tests et qualité

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Visualisation avec PlantUML

```bash
docker pull plantuml/plantuml-server:jetty
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

Ouvrir http://localhost:8080/ et coller le contenu de
graph/04_common_parts.puml pour voir le diagramme.

## Structure du projet

```
phone_trie/
├── Cargo.toml
├── README.md
├── data/04_common_parts.json     # données de test (NE PAS RENOMMER)
├── graph/                         # sorties PlantUML
├── src/
│   ├── lib.rs                     # #![forbid(unsafe_code)]
│   ├── main.rs                    # CLI
│   ├── contact.rs                 # struct Contact
│   ├── loader.rs                  # lecture JSON
│   ├── trie.rs                    # trie from scratch
│   └── plantuml.rs                # export MindMap
└── tests/
    └── tests.rs                   # tests d'intégration
```

## Choix d'implémentation

- Alphabet fixe : chaque nœud a un tableau [Option<Box<TrieNode>>; 10],
  indexé par le chiffre. Accès O(1) par chiffre.
- Plusieurs noms par nœud (Vec<String>) pour gérer les contacts qui
  partagent exactement le même numéro.
- seules l'insertion et le parcours sont implémentés.
- #![forbid(unsafe_code)] interdit tout bloc unsafe à la compilation.