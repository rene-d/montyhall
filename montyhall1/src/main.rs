// montyhall.rs

const PORTES: u32 = 3;

static mut SEED: u32 = 1;

// Initialise le générateur de nombres aléatoires
fn srand() {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    unsafe {
        SEED = seed;
    }
}

// Générateur congruentiel linéaire
fn rand() -> u32 {
    unsafe {
        SEED = (SEED.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        SEED
    }
}

// Retourne un nombre entre 1 et PORTES
fn choix() -> u32 {
    rand() % PORTES + 1
}

// Retourne un nombre entre 1 et PORTES avec `a` exclu
fn choix_autre(a: u32) -> u32 {
    loop {
        let r = choix();
        if r != a {
            return r;
        }
    }
}

// Retourne un nombre entre 1 et PORTES avec `a` et `b` exclus
fn choix_autre_autre(a: u32, b: u32) -> u32 {
    loop {
        let r = choix();
        if r != a && r != b {
            return r;
        }
    }
}

fn main() {
    srand();

    let voiture = choix();
    let joueur = choix();

    let presentateur = if joueur == voiture {
        // le joueur choisit la porte à voiture
        choix_autre(voiture)
    } else {
        // le joueur choisit une porte à chèvre
        choix_autre_autre(voiture, joueur)
    };

    // Le joueur ne change pas de porte
    let sans_changement = joueur == voiture;

    // Le joueur change de porte
    let second = choix_autre_autre(presentateur, joueur);
    let avec_changement = second == voiture;

    println!("la voiture est derrière la porte   : {}", voiture);
    println!("le joueur choisit la porte         : {}", joueur);
    println!("le présentateur ouvre la porte     : {}", presentateur);
    println!("le joueur gagne s'il ne change pas : {}", sans_changement);
    println!("le joueur gagne s'il change        : {}", avec_changement);
}
