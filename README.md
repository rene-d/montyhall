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
- accès concurrents aux variables globales (`static`) et usage vraiment pas recommandé de `unsafe`.

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
