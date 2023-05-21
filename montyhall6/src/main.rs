//! Simulation du problème de [Monty Hall](https://fr.wikipedia.org/wiki/Problème_de_Monty_Hall)
//!
//! Soient trois portes, l'une cache une voiture, les deux autres des chèvres.
//! - Le présentateur connaît ce qu'il y a derrière les portes.
//! - Le joueur choisit une des portes, mais ne l'ouvre pas.
//! - Le présentateur ouvre une autre porte, mais pas celle qui cache la voiture.
//! - Le joueur a le choix de changer son choix de porte à ouvrir.
//!
//! Le présentateur propose au candidat de changer son choix de porte à ouvrir définitivement.
//!
//! Le joueur doit-il changer son choix ou maintenir son choix initial (si toutefois il préfère
//! gagner la voiture qu'une chèvre) ?
//!
//              ,,~~--___---,
//             /            .~,
//       /  _,~             )
//      (_-(~)   ~, ),,,(  /'
//       Z6  .~`' ||     \ |
//       /_,/     ||      ||
// ~~~~~~~~~~~~~~~W`~~~~~~W`~~~~~~~~~
//

use clap::Parser;
use fraction::Fraction;
use rand::prelude::*;

/// Structure des arguments de ligne de commande.
#[derive(Parser)]
struct Args {
    /// nombre de portes
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(3..=100))]
    portes: u32,

    /// nombre de tours à simuler
    #[arg(short, long, default_value_t = 100_000)]
    tours: u32,

    /// nombre de portes ouvertes par le présentateur
    #[arg(short, long, default_value_t=1, value_parser = clap::value_parser!(u32).range(0..=98))]
    ouverture: u32,
}

/// Implémente le jeu de Monty Hall.
#[derive(Debug)] // pour afficher la structure
struct Jeu {
    /// Générateur de nombres aléatoires.
    gna: rand::rngs::ThreadRng,
    /// Nombre de tours de la simulation.
    tours: u32,
    /// Nombre de portes du jeu.
    portes: u32,
    /// Nombre de portes ouvertes par le présentateur.
    ouverture: u32,
}

impl Jeu {
    /// Initialise la structure `Jeu`
    fn new(tours: u32, portes: u32, ouverture: u32) -> Self {
        if ouverture + 1 >= portes {
            panic!("Erreur: le nombre de portes ouvertes par le présentateur est trop grand");
        }

        Self {
            gna: rand::thread_rng(),
            tours: tours,
            portes: portes,
            ouverture: ouverture,
        }
    }

    /// Retourne un numéro de porte, sauf éventuellement certains numéros.
    fn choix_sauf(&mut self, exclu: &[u32]) -> u32 {
        let exclu = exclu
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();

        if exclu.len() == self.portes as usize {
            panic!("aucun choix possible, exclu={exclu:?}");
        }

        loop {
            let r = self.gna.gen_range(1..=self.portes);
            if !exclu.contains(&r) {
                return r;
            }
        }
    }

    /// Effectue un tour du jeu. Retourne si le joueur gagne
    /// sans changer et en changeant son choix initial.
    /// Retourne deux `bool` qui indiquent si le joueur a gagné sans ou avec changement de porte.
    fn tour(&mut self) -> (bool, bool) {
        let voiture = self.choix_sauf(&[]);
        let joueuse = self.choix_sauf(&[]);

        if joueuse == voiture {
            // la 💃 a choisi la 🚗 lors du premier choix
            // elle ne peut pas gagner en changeant de 🚪
            return (true, false);
        }

        // le 🤵‍♂️ ouvre des 🚪 (sauf celle de la 👩‍🦰 et celle avec la 🚗)
        let mut exclu = vec![joueuse, voiture];

        for _ in 0..self.ouverture {
            let ouverte = self.choix_sauf(&exclu);
            exclu.push(ouverte);
        }

        // on enlève la 🚗 des valeurs interdites (i.e. on laisse la possibilité de gagner!)
        exclu.retain(|&porte| porte != voiture);

        // la 💃 choisit une autre 🚪
        let joueur = self.choix_sauf(&exclu);

        if joueur == voiture {
            // la 💃 a trouvé la 🚗 lors de son deuxième choix
            return (false, true);
        }

        // la 🤦‍♀️ repart avec une 🐐
        (false, false)
    }

    /// Lance la simulation sur plusieurs tours.
    /// Retourne les probabilités de trouver la voiture sans et avec changement de porte.
    fn simule(&mut self) -> (f64, f64) {
        let mut avec_changement = 0;
        let mut sans_changement = 0;

        for _ in 0..self.tours {
            match self.tour() {
                (true, false) => {
                    sans_changement += 1;
                }
                (false, true) => {
                    avec_changement += 1;
                }
                (false, false) => (),
                _ => unreachable!(), // on ne peut pas gagner à tous les coups 😃
            }
        }

        let p = f64::from(sans_changement) / f64::from(self.tours);
        let q = f64::from(avec_changement) / f64::from(self.tours);

        (p, q)
    }
}

fn main() {
    let args = Args::parse();

    let mut jeu = Jeu::new(args.tours, args.portes, args.ouverture);
    println!("paramètres: {jeu:?}");

    let (p, q) = jeu.simule();
    println!("simulation:\n  sans={p:<7.5}\n  avec={q:<7.5}");

    let p = Fraction::from(1) / Fraction::from(args.portes);
    let q = p * Fraction::from(args.portes - 1) / Fraction::from(args.portes - args.ouverture - 1);
    println!("\x1B[3mcalcul:\n  sans={p:<7.5}  {p}\n  avec={q:<7.5}  {q}\x1B[0m",);

    if args.portes == 3 && args.ouverture == 1 {
        println!(
            r"
Au début, on a une chance sur trois de trouver la voiture. Si on ne change pas
de porte, la probabilité ne change évidemment pas, puisqu'on n'a rien changé,
et est : 1/3.

En revanche, on a deux chances sur trois de trouver une chèvre au début. Si on
a choisi la porte avec une chèvre, le présentateur ne peut ouvrir que celle
avec l'autre chèvre. Donc la voiture est nécessairement derrière la troisième
porte. Ainsi, en changeant de porte, on a la même probabilité de trouver la
voiture au deuxième tour que trouver une chèvre au premier tour, soit : 2/3.

Bêêê 🐐🐐🐐
"
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // tous les choix possibles
    #[test]
    fn test_exclu_0() {
        let mut jeu = Jeu::new(1, 3, 1);

        let r = 1..=3;
        for _ in 0..100 {
            let t = jeu.choix_sauf(&[]);
            assert!(r.contains(&t));
        }
    }

    /// deux choix possibles
    #[test]
    fn test_exclu_1() {
        let mut jeu = Jeu::new(1, 5, 1);

        let r = 1..=5;
        for _ in 0..100 {
            let t = jeu.choix_sauf(&[1]);
            assert!(r.contains(&t));
            assert_ne!(t, 1);
        }
    }

    /// un seul choix possible
    #[test]
    fn test_exclu_2() {
        let mut jeu = Jeu::new(1, 3, 1);

        for _ in 0..100 {
            let t = jeu.choix_sauf(&[1, 2]);

            assert_ne!(t, 1);
            assert_ne!(t, 2);
            assert_eq!(t, 3);
        }
    }

    /// paramètre ouverture trop grand
    #[test]
    #[should_panic]
    fn test_exclu_3() {
        Jeu::new(1, 3, 2);
    }

    /// aucun choix possible de porte
    #[test]
    #[should_panic]
    fn test_exclu_4() {
        let mut jeu = Jeu::new(1, 3, 0);

        jeu.choix_sauf((1..=3).collect::<Vec<_>>().as_slice());
    }
}
