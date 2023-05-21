// montyhall.rs

use clap::Parser;
use fraction::Fraction;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(3..=100))]
    portes: u32,

    #[arg(short, long, default_value_t = 100000)]
    tours: u32,
}

struct Jeu {
    gna: rand::rngs::ThreadRng,
    portes: u32,
}

impl Jeu {
    fn new(portes: u32) -> Self {
        Self {
            gna: rand::thread_rng(),
            portes,
        }
    }

    // implémentation générique, mais moins efficace que les cas particuliers
    fn choix_exclu(&mut self, exclu: &[u32]) -> u32 {
        let mut r = self.gna.gen_range(1..=self.portes - exclu.len() as u32);

        let mut exclu_sorted = exclu.iter().collect::<Vec<_>>();
        exclu_sorted.sort();

        for i in exclu_sorted {
            if &r >= i {
                r += 1;
            }
        }

        r
    }

    fn tour(&mut self) -> (bool, bool) {
        let voiture = self.choix_exclu(&[]);
        let joueur = self.choix_exclu(&[]);

        let presentateur = if joueur == voiture {
            // le joueur choisit la porte à voiture
            self.choix_exclu(&[voiture])
        } else {
            // le joueur choisit une porte à chèvre
            self.choix_exclu(&[voiture, joueur])
        };

        // Le joueur ne change pas de porte
        let sans_changement = joueur == voiture;

        // Le joueur change de porte
        let second = self.choix_exclu(&[presentateur, joueur]);
        let avec_changement = second == voiture;

        return (sans_changement, avec_changement);
    }
}

fn main() {
    let args = Args::parse();

    let mut jeu = Jeu::new(args.portes);

    let mut avec_changement = 0u32;
    let mut sans_changement = 0u32;

    for _ in 0..args.tours {
        match jeu.tour() {
            (true, true) => {
                sans_changement += 1;
                avec_changement += 1;
            }
            (true, false) => {
                sans_changement += 1;
            }
            (false, true) => {
                avec_changement += 1;
            }
            (false, false) => (),
        }
    }

    let p = f64::from(sans_changement) / f64::from(args.tours);
    let q = f64::from(avec_changement) / f64::from(args.tours);
    println!(
        "portes={} tours={} sans={p:<7.5} avec={q:<7.5}",
        args.portes, args.tours
    );

    let _1 = Fraction::from(1);
    let p = _1 / args.portes;
    let q = _1 / args.portes * (_1 + _1 / (args.portes - 2));
    println!(
        "\x1B[3mportes={} tours={} sans={p:<7.5} avec={q:<7.5}\x1B[0m",
        args.portes, args.tours
    );
}
