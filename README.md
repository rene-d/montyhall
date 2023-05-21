# Monty Hall

Initiation au langage Rust via une simulation du problème de [Monty Hall](https://fr.wikipedia.org/wiki/Problème_de_Monty_Hall).

Prérequis:

- une installation fonctionnelle de Rust.
- un accès internet ou _a minima_ aux crates [rand](https://crates.io/crates/rand), [clap](https://crates.io/crates/clap), [fraction](https://crates.io/crates/fraction).
- des connaissnces en programmation.

## Etape 1 : introduction

Concepts abordés:

- éléments du langage
- règles de nommage, `cargo fmt`, `cargo clippy`
- débordement des types entiers (`u32`)
- accès concurrents aux variables globales (`static`) et usage pas recommandé de `unsafe`.

```shell
cargo fmt
cargo clippy -- -A clippy::pedantic -A clippy::all --no-deps
```

## Etape 2

Concepts abordés:

- définition de structures (`struct`)
- les plages

## Etape 3

Concepts abordés:

- utilisation d'une crate externe (`rand`)
- les traits

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

- documentation
- tests
