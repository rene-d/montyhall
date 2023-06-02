# Monty Hall

Initiation au langage Rust via une simulation du problème de [Monty Hall](https://fr.wikipedia.org/wiki/Problème_de_Monty_Hall).

Prérequis:

- une installation fonctionnelle de Rust.
- un accès internet ou _a minima_ aux crates [rand](https://crates.io/crates/rand), [clap](https://crates.io/crates/clap), [fraction](https://crates.io/crates/fraction).
- des connaissnces en programmation.

## Introduction

Présentation de Rust. Où trouver de l'information, où se former, comment progresser.

Installer Rust sur un ordinateur : [rustup](https://rustup.rs/)

Installer et utiliser un IDE qui gère Rust : [Visual Studio Code](https://code.visualstudio.com/) + [rust-analyzer](https://rust-analyzer.github.io/) + [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) + [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

Cargo, le CLI à tout faire :

- création d'un projet vide : `cargo new --vcs none hello-world`
- compiler : `cargo build`, `cargo build --release`
- exécuter : `cargo run`, `cargo run --release`
- formater (linter) : `cargo fmt`
- vérifier (SAST) : `cargo clippy`

## Les préliminaires

- afficher du texte : `println!("hello");`
- déclarer une variable : `let a = 1;`
- afficher une variable : `println!("a={}", a);` `println!("a={a}");`
- déclarer une variable mutable : `let mut a = 1; a += 1;`
- les types numériques : `i32`, `u8`, `f64`, etc.
- les structures de contrôle : `if`, `for`, `while`, `loop`, etc.
- déclarer une fonction : `fn fonction(arg: type) -> type { }`

## Etape 1

Concepts abordés:

- règles de nommage, `cargo fmt`, `cargo clippy`
- débordement des types entiers (`u32`)
- accès concurrents aux variables globales (`static`) et usage _vraiment_ pas recommandé de `unsafe`

Tâches:

- écrire une fonction `rand()` qui implémente un [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator)
- écrire une fonction `choix()` qui retourne un nombre entre `1` et `PORTES`, constante fixée à `3`
- écrire l'algorithme, on ajoutera des fonctions `choix_autre()` qui retournent des nombres aléatoires dans la plage `[1, PORTES]` sauf certains
- afficher les résultats avec `println!()`

## Etape 2

Concepts abordés:

- définition de structures (`struct`)
- les plages

## Etape 3

Concepts abordés:

- utilisation d'un crate externe (`rand`)

## Etape 4

Concepts abordés:

- analyse des options de ligne de commande
- le mélange impossible entre types (`f64` et `u32` ici)

## Etape 5

Concepts abordés:

- introduction aux vecteurs
- mot-clé `match`
- crate `fraction`

## Etape 6

Concepts abordés:

- suite mot-clé `match` et `Option<T>`
- documentation
- tests
- couverture
