// montyhall.rs

use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 100_000)]
    tours: u32,

    #[arg(short, long, default_value_t = 3)]
    portes: u32,
}

struct Gna {
    gna: rand::rngs::ThreadRng,
    portes: u32,
}

impl Gna {
    fn new(portes: u32) -> Self {
        Self {
            gna: rand::thread_rng(),
            portes: portes,
        }
    }

    fn randrange(&mut self, a: u32, b: u32) -> u32 {
        self.gna.gen_range(a..=b)
    }

    fn choix(&mut self) -> u32 {
        self.randrange(1, self.portes)
    }

    fn choix_sauf(&mut self, a: u32, b: Option<u32>) -> u32 {
        let mut r: u32; // le compilateur va vérifier que la variable est toujours initialisée

        if b.is_none() {
            r = self.randrange(1, self.portes - 1);
            if r >= a {
                r += 1;
            }
        } else {
            let b = b.unwrap(); // shadowing de b: on transforme b en la valeur qu'il contient
            
            let (a, b) = if b > a { (a, b) } else { (b, a) }; // a=min, b=max
            r = self.randrange(1, self.portes - 2);
            if r >= a {
                r += 1;
            }
            if r >= b {
                r += 1;
            }
        }

        r // retour de la valeur, équivalent à return r;
    }
}

fn main() {
    let args = Args::parse();

    let mut gna = Gna::new(args.portes);

    let mut avec_changement = 0u32;
    let mut sans_changement = 0u32;

    for _ in 0..args.tours {
        let voiture = gna.choix();
        let joueur = gna.choix();

        let presentateur = if joueur == voiture {
            // le joueur choisit la porte à voiture
            gna.choix_sauf(voiture, None)
        } else {
            // le joueur choisit une porte à chèvre
            gna.choix_sauf(voiture, Some(joueur))
        };

        // Le joueur ne change pas de porte
        if joueur == voiture {
            sans_changement += 1;
        }

        // Le joueur change de porte
        let second = gna.choix_sauf(presentateur, Some(joueur));
        if second == voiture {
            avec_changement += 1;
        }
    }

    let p = f64::from(sans_changement) / f64::from(args.tours);
    let q = f64::from(avec_changement) / f64::from(args.tours);
    println!(
        "portes={} tours={} sans={p:.5} avec={q:.5}",
        args.portes, args.tours
    );
}
