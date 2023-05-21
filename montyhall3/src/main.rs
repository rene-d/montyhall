// montyhall.rs

use rand::prelude::*;

const PORTES: u32 = 5;
const TOURS: u32 = 100_000;

struct Gna {
    gna: rand::rngs::ThreadRng,
}

impl Gna {
    fn new() -> Self {
        Self {
            gna: rand::thread_rng(),
        }
    }

    fn randrange(&mut self, a: u32, b: u32) -> u32 {
        self.gna.gen_range(a..=b)
    }

    fn choix(&mut self) -> u32 {
        self.randrange(1, PORTES)
    }

    fn choix_autre(&mut self, a: u32) -> u32 {
        let mut r = self.randrange(1, PORTES - 1);
        if r >= a {
            r += 1;
        }
        r
    }

    fn choix_autre_autre(&mut self, a: u32, b: u32) -> u32 {
        let (a, b) = if b > a { (a, b) } else { (b, a) };
        let mut r = self.randrange(1, PORTES - 2);
        if r >= a {
            r += 1;
        }
        if r >= b {
            r += 1;
        }
        r
    }
}

fn main() {
    let mut gna = Gna::new();

    let mut avec_changement = 0u32;
    let mut sans_changement = 0u32;

    for _ in 0..TOURS {
        let voiture = gna.choix();
        let joueur = gna.choix();

        let presentateur = if joueur == voiture {
            // le joueur choisit la porte à voiture
            gna.choix_autre(voiture)
        } else {
            // le joueur choisit une porte à chèvre
            gna.choix_autre_autre(voiture, joueur)
        };

        // Le joueur ne change pas de porte
        if joueur == voiture {
            sans_changement += 1;
        }

        // Le joueur change de porte
        let second = gna.choix_autre_autre(presentateur, joueur);
        if second == voiture {
            avec_changement += 1;
        }
    }

    println!("nombre de portes={PORTES}");
    println!("nombre de tours={TOURS}");
    println!("victoires sans changement={sans_changement}");
    println!("victoires avec changement={avec_changement}");
}
