# Monty Hall

Initiation au langage [Rust](https://www.rust-lang.org/) via une simulation du problème de [Monty Hall](https://fr.wikipedia.org/wiki/Problème_de_Monty_Hall).

![goat](rustgoat.png)

Prérequis:

- une installation fonctionnelle de Rust.
- un accès internet ou _a minima_ aux _crates_ [rand](https://crates.io/crates/rand), [clap](https://crates.io/crates/clap), [fraction](https://crates.io/crates/fraction).
- des connaissances en programmation.

## Introduction

Présentation de Rust.

- Langage moderne
- Compilé natif et multi-plateformes
- Syntaxe proche C/C++ avec quelques références aux langages fonctionnels
- Multi-paradigmes (impératif et fonctionnel)
- Typage statique fort sans conversion implicite
- Inférence de types
- La gestion obligatoire des erreurs
- Gestion mémoire sans _garbage collector_ vérifiée à la compilation, le **pointeur nul** n'existe pas
- Gestion de la concurrence intégrée au langage
- Eco-système complet, riche et vivant
- Domaine d'utilisation très vaste: du kernel au back-end web et même WASM, en passant par des logiciels système, des outils, des systèmes complets, des algorithmes sensibles/performants, évidemment Firefox, etc.

Où trouver de l'information, où se former, comment progresser.

- Le [site officiel](https://www.rust-lang.org/fr/learn)
- Le [bac à sable](https://play.rust-lang.org/)
- Formations en ligne [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/), [Débutez en Rust](https://www.udemy.com/course/debutez-en-rust/), etc.
- GitHub, StackOverflow, blogs, bref Internet
- Pratiquer ! (suggestion: [Advent of Code](https://adventofcode.com))

Installer Rust sur un ordinateur: [rustup](https://rustup.rs/)

Alternative (avec un conteneur [Docker](https://hub.docker.com/_/rust)) : `docker run --rm -it -v "$PWD":/workspaces -w /workspaces rust`

Installer et utiliser un IDE qui gère Rust : [Visual Studio Code](https://code.visualstudio.com/) + [rust-analyzer](https://rust-analyzer.github.io/) + [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) + [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

[Cargo](https://doc.rust-lang.org/cargo/), le [CLI](https://en.wikipedia.org/wiki/Command-line_interface) à tout faire :

- créer un projet vide:  `cargo init --vcs none`, `cargo new hello-world`
- compiler: `cargo build`, `cargo build --release`
- exécuter: `cargo run`, `cargo run --release`
- tester: `cargo test`
- formater (linter): `cargo fmt`
- vérifier (SAST): `cargo clippy`

Et bien d'autres actions.

## Les préliminaires

A faire dans le [playground Rust](https://play.rust-lang.org/).

- déclarer une fonction: `fn main() { }`
- afficher du texte: `println!("hello");`
- déclarer une variable: `let a = 1;`
- afficher une variable: `println!("a={}", a);` `println!("a={a}");`
- déclarer une variable mutable: `let mut a = 1; a += 1;`
- les types numériques: `i32`, `u8`, `f64`, etc.
- les structures de contrôle: `if`, `for`, `while`, `loop`, etc.
- déclarer une fonction avec paramètre: `fn fonction(arg: type) -> type { }`
- les types `&str` et `String`

## Etape 0

Le problème de Monty Hall est un [biais cognitif](https://fr.wikipedia.org/wiki/Biais_cognitif): la solution est contre-intuitive. Nous allons écrire un programme qui simule le jeu télévisé qui a inspiré ce problème et observer le biais. Il y a évidemment des démonstrations de la formule mathématique du résultat de la simulation (cf. l'article [Wikipedia](https://fr.wikipedia.org/wiki/Probl%C3%A8me_de_Monty_Hall)).

Principe du jeu:

- derrière 🚪🚪🚪 portes, il y a 🚗 voiture et 🐐🐐 chèvres
- le joueur choisit 🚪 porte
- le présentateur ouvre 🚪 porte derrière laquelle il y a 🐐 chèvre et demande au joueur s'il veut changer de porte
- _le joueur doit-il ou non changer son choix initial s'il veut maximiser ses chances de gagner la voiture ?_

Eléments de la simulation:

- tirage de nombre aléatoire
- fonction
- expression conditionnelle
- boucle
- affichage de texte, de variable

## Etape 1

A faire dans l'environnement de développement (Codespaces ou autre).

Ecrire la simulation d'une partie du jeu. On mettra le nombre de portes en constante: `const PORTES: u32 = 3;`.

1. choix de la porte avec la voiture
2. choix du joueur
3. choix de la porte ouverte par le présentateur
4. est-ce que le joueur a gagné sans changer son choix ?
5. est-ce que le joueur a gagné après avoir changé son choix ?

Concepts abordés:

- règles de nommage, `cargo fmt`, `cargo clippy`
- débordement des types entiers (`u32`)
- accès concurrents aux variables globales (`static`) et usage _vraiment_ pas recommandé de `unsafe`

Sous-étapes / tâches:

- écrire une fonction `rand()` qui implémente un [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator) retournant une valeur de type `u32`. On utilisera une _seed_ pour que `rand()` retourne un nombre différent à chaque fois. On pourra s'inspirer des implémentations en C.
- écrire une fonction `choix()` qui retourne un nombre entre `1` et `PORTES`, constante fixée à `3`.
- écrire l'algorithme, on ajoutera une fonction `choix_autre()` qui retourne des nombres aléatoires dans la plage `[1, PORTES]` sauf certaines valeurs.
- afficher les données et résultats avec `println!()`

On peut éventuellement initialiser la variable _seed_ avec le snippet suivant pour rendre le comportement du programme un peu moins prévisible:

```rust
let alea = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .subsec_nanos();
```

Exemple de sortie attendue:

```text
la voiture est derrière la porte   : 3
le joueur choisit la porte         : 2
le présentateur ouvre la porte     : 1
le joueur gagne s'il ne change pas : false
le joueur gagne s'il change        : true
```

## Etape 2

Améliorer la simulation pour ajouter un nombre de parties afin de voir la tendance: `const TOURS: u32 = 100_000;`.

Améliorer le générateur de nombre aléatoire pour ne plus dépendre de la variable globale _seed_:

```rust
struct Gna {
    seed: u32;
}

impl Gna {
    fn new() -> Self {
        Self { seed: 1 }
    }

    fn rand() -> u32 {
        ...
    }
}
```

Concepts abordés:

- définition de structures (`struct`)
- les plages

Exemple de sortie attendue:

```text
nombre de portes=3
nombre de tours=100000
victoires sans changement=33311
victoires avec changement=66689
```

## Etape 3

Améliorer les tirages aléatoires en utilisant la bibliothèque (_crate_) [rand](https://docs.rs/rand/latest/rand/).

Concepts abordés:

- utilisation d'un _crate_ externe

Exemple de sortie (en faisant varier le nombre de portes):

```text
nombre de portes=5
nombre de tours=100000
victoires sans changement=20021
victoires avec changement=26703
```

## Etape 4

Rendre paramétrable les constantes `TOURS` et `PORTES` en utilisant les valeurs en option de la ligne de commande. Afficher les probabilités de trouver la voiture, sans et avec changement.

Utiliser [clap](https://crates.io/crates/clap) et sa _feature_ `derive`.

On écrira une fonction `choix_sauf(&mut self, a: u32, b: Option<u32>) -> u32` permettant de choisir un nombre entre 1 et `self.portes` sauf `a`, et `b` s'il contient une valeur.

Concepts abordés:

- analyse des options de ligne de commande
- type `Option<T>`, `unwrap()`, `None`
- le mélange implicite impossible entre types (`f64` et `u32` ici)

Exemple de sortie:

```text
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `/workspaces/montyhall/target/debug/montyhall --help`
Usage: montyhall [OPTIONS]

Options:
  -t, --tours <TOURS>    [default: 100000]
  -p, --portes <PORTES>  [default: 3]
  -h, --help             Print help

$ /workspaces/montyhall/target/debug/montyhall -t 1234567
portes=3 tours=1234567 sans=0.33345 avec=0.66655
```

## Etape 5

Rendre paramétrable le nombre de portes ouvertes par le présentateur et adapter l'algorithme.

Vérifier que le résultat de la simulation converge bien vers la valeur suivante lorsque le joueur change son choix:

$$
{\frac {1}{n}}{\frac {n-1}{n-p-1}}
$$

où $n$ est le nombre total de portes et $p$ celles ouvertes par le présentateur.

A comparer avec la probabilité de gain avec le choix initial, qui est de manière assez évidente $\frac {1}{n}$.

La structure `Jeu` contiendra les paramètres et le générateur de nombres aléatoires, ainsi que les trois méthodes:

- `choix_sauf(&mut self, exclu: &[u32]) -> u32` retournant un nombre entre 1 et `portes` sauf les valeurs de `exclu`
- `tour(&mut self) -> (bool, bool)` simulant une partie du jeu et retournant si le joueur a gagné sans et avec changement
- `simule(&mut self) -> (f64, f64)` simulant les parties et renvoyant les probabilités de gain

Concepts abordés:

- introduction aux vecteurs
- mot-clé `match`
- _crate_ `fraction`

## Etape 6

Ajout de la documentation, tests unitaires.

Concepts abordés:

- tags documentaires `//!` et `///`
- tests unitaires `#[cfg(test)]` et `#[test]`
- couverture (cf. script dans la branche [solutions](../solutions/cov.sh))
