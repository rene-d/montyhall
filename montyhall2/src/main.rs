// montyhall.rs

const PORTES: u32 = 3;
const TOURS: u32 = 1000000;

struct Gna {
    seed: u32,
}

impl Gna {
    fn new() -> Self {
        Self { seed: 1 }
    }

    fn rand(&mut self) -> u32 {
        self.seed = (self.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        self.seed
    }

    fn randrange(&mut self, r: std::ops::RangeInclusive<u32>) -> u32 {
        let start = r.start();
        let end = r.end();
        self.rand() % (end - start + 1) + start
    }

    fn choix(&mut self) -> u32 {
        self.randrange(1..=PORTES)
    }

    fn choix_autre(&mut self, a: u32) -> u32 {
        let mut r = self.randrange(1..=PORTES - 1);
        if r >= a {
            r += 1;
        }
        r
    }

    fn choix_autre_autre(&mut self, a: u32, b: u32) -> u32 {
        let (a, b) = if b > a { (a, b) } else { (b, a) };
        let mut r = self.randrange(1..=PORTES - 2);
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
    let mut rng = Gna::new();

    let mut avec_changement = 0u32;
    let mut sans_changement = 0u32;

    for _ in 0..TOURS {
        let voiture = rng.choix();
        let joueur = rng.choix();

        let presentateur = if joueur == voiture {
            // le joueur choisit la porte à voiture
            rng.choix_autre(voiture)
        } else {
            // le joueur choisit une porte à chèvre
            rng.choix_autre_autre(voiture, joueur)
        };

        // Le joueur ne change pas de porte
        if joueur == voiture {
            sans_changement += 1;
        }

        // Le joueur change de porte
        let second = rng.choix_autre_autre(presentateur, joueur);
        if second == voiture {
            avec_changement += 1;
        }
    }

    let p = f64::from(sans_changement) / f64::from(TOURS);
    let q = f64::from(avec_changement) / f64::from(TOURS);
    println!("portes={PORTES} tours={TOURS} sans={p:.5} avec={q:.5}");
}
