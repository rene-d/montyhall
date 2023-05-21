// montyhall.rs

use clap::Parser;
use fraction::Fraction;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
    /// nombre de portes
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(3..=100))]
    portes: u32,

    /// nombre de tours à simuler
    #[arg(short, long, default_value_t = 100_000)]
    tours: u32,

    /// nombre de portes ouvertes par le présentateur
    #[arg(short, long, default_value_t=1, value_parser = clap::value_parser!(u32).range(1..=98))]
    ouverture: u32,
}

#[derive(Debug)] // pour afficher la structure
struct Jeu {
    gna: rand::rngs::ThreadRng,
    tours: u32,
    portes: u32,
    ouverture: u32,
}

impl Jeu {
    fn new(tours: u32, portes: u32, ouverture: u32) -> Self {
        Self {
            gna: rand::thread_rng(),
            tours: tours,
            portes: portes,
            ouverture: ouverture,
        }
    }

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

    if args.ouverture + 1 >= args.portes {
        eprintln!("Erreur: le nombre de portes ouvertes par le présentateur est trop grand");
        std::process::exit(1);
    }

    let mut jeu = Jeu::new(args.tours, args.portes, args.ouverture);
    println!("paramètres: {jeu:?}");

    let (p, q) = jeu.simule();
    println!("simulation:\n  sans={p:<7.5}\n  avec={q:<7.5}");

    let p = Fraction::from(1) / Fraction::from(args.portes);
    let q = p * Fraction::from(args.portes - 1) / Fraction::from(args.portes - args.ouverture - 1);
    println!("\x1B[3mcalcul:\n  sans={p:<7.5}  {p}\n  avec={q:<7.5}  {q}\x1B[0m",);
}
