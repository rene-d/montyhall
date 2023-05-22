//! Simulation du problème de [Monty Hall](https://fr.wikipedia.org/wiki/Problème_de_Monty_Hall)
//!
//! Soient trois portes, l'une cache une voiture, les deux autres une chèvre.
//! - Le présentateur connaît ce qu'il y a derrière les portes.
//! - Le joueur choisit une des portes, mais ne l'ouvre pas.
//! - Le présentateur ouvre une autre porte, mais pas celle qui cache la voiture.
//! - Le joueur a le choix de changer son choix de porte à ouvrir.
//!
//! Le présentateur propose au candidat de changer son choix de porte à ouvrir définitivement.
//!
//! Le joueur doit-il changer son choix ou maintenir son choix initial ?
//!

use clap::Parser;
use fraction::Fraction;
use rand::prelude::*;

/// Structure des arguments de ligne de commande.
#[derive(Parser)]
struct Args {
    /// Nombre de portes (au minimum 3).
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(3..=100))]
    portes: u32,

    /// Nombre de tirage de simulation.
    #[arg(short, long, default_value_t = 100_000)]
    tours: u32,
}

/// Implémente le jeu de Monty Hall.
struct Jeu {
    /// Générateur de nombres aléatoires.
    gna: rand::rngs::ThreadRng,
    /// Nombre de portes du jeu.
    portes: u32,
}

impl Jeu {
    fn new(portes: u32) -> Self {
        Self {
            gna: rand::thread_rng(),
            portes,
        }
    }

    /// Retourne un numéro de porte, sauf éventuellement certains numéros.
    fn choix_exclu(&mut self, a: Option<u32>, b: Option<u32>) -> u32 {
        let n = self.portes - (if a.is_some() { 1 } else { 0 }) - (if b.is_some() { 1 } else { 0 });

        let mut r = self.gna.gen_range(1..=n);

        match (a, b) {
            (Some(a), None) => {
                if r >= a {
                    r += 1;
                }
            }
            (None, Some(b)) => {
                if r >= b {
                    r += 1;
                };
            }
            (Some(a), Some(b)) => {
                if r >= a.min(b) {
                    r += 1;
                }
                if r >= a.max(b) {
                    r += 1;
                }
            }
            _ => (),
        };

        r
    }

    /// Effectue un tour du jeu. Retourne si le joueur gagne
    /// sans changer et en changeant son choix initial.
    /// Retourne deux `bool` qui indiquent si le joueur a gagné sans ou avec changement de porte.
    fn tour(&mut self) -> (bool, bool) {
        let voiture = self.choix_exclu(None, None);
        let joueur = self.choix_exclu(None, None);

        let presentateur = if joueur == voiture {
            // le joueur choisit la porte à voiture
            self.choix_exclu(Some(voiture), None)
        } else {
            // le joueur choisit une porte à chèvre
            self.choix_exclu(Some(voiture), Some(joueur))
        };

        // Le joueur ne change pas de porte
        let sans_changement = joueur == voiture;

        // Le joueur change de porte
        let second = self.choix_exclu(Some(presentateur), Some(joueur));
        let avec_changement = second == voiture;

        (sans_changement, avec_changement)
    }
}

fn main() {
    let args = Args::parse();

    let mut jeu = Jeu::new(args.portes);

    let mut avec_changement = 0u32;
    let mut sans_changement = 0u32;

    for _ in 0..args.tours {
        let (sans, avec) = jeu.tour();

        if sans {
            sans_changement += 1;
        }
        if avec {
            avec_changement += 1;
        }
    }

    let p = f64::from(sans_changement) / f64::from(args.tours);
    let q = f64::from(avec_changement) / f64::from(args.tours);
    println!(
        "portes={} tours={} sans={p:<7.5} avec={q:<7.5}",
        args.portes, args.tours
    );

    let un = Fraction::from(1);
    let p = un / args.portes;
    let q = un / args.portes * (un + un / (args.portes - 2));
    println!(
        "\x1B[3mportes={} tours={} sans={p:<7.5} avec={q:<7.5}\x1B[0m",
        args.portes, args.tours
    );
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test de la fonction choix_exclu()
    #[test]
    fn test_exclu_0() {
        let mut jeu = Jeu::new(3);

        let r = 1..=3;
        for _ in 0..100 {
            let t = jeu.choix_exclu(None, None);
            assert!(r.contains(&t));
        }
    }

    /// Test de la fonction choix_exclu()
    #[test]
    fn test_exclu_1() {
        let mut jeu = Jeu::new(3);

        let r = jeu.choix_exclu(Some(1), None);
        assert!(r == 2 || r == 3);

        let r = jeu.choix_exclu(Some(2), None);
        assert!(r == 1 || r == 3);

        let r = jeu.choix_exclu(Some(3), None);
        assert!(r == 1 || r == 2);

        let r = jeu.choix_exclu(None, Some(1));
        assert!(r == 2 || r == 3);

        let r = jeu.choix_exclu(None, Some(2));
        assert!(r == 1 || r == 3);

        let r = jeu.choix_exclu(None, Some(3));
        assert!(r == 1 || r == 2);
    }

    /// Test de la fonction choix_exclu(Some())
    #[test]
    fn test_exclu_2() {
        let mut jeu = Jeu::new(3);

        assert_eq!(jeu.choix_exclu(Some(1), Some(2)), 3);
        assert_eq!(jeu.choix_exclu(Some(2), Some(1)), 3);
        assert_eq!(jeu.choix_exclu(Some(1), Some(3)), 2);
        assert_eq!(jeu.choix_exclu(Some(3), Some(1)), 2);
        assert_eq!(jeu.choix_exclu(Some(2), Some(3)), 1);
        assert_eq!(jeu.choix_exclu(Some(3), Some(2)), 1);
    }
}
