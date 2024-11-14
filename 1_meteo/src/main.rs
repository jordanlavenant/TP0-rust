#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
enum Alerte {
    Vent,
    Inondation,
}

fn affiche_alerte(a: Alerte) -> &'static str {
    match a {
        Alerte::Vent => "Alerte vent fort",
        Alerte::Inondation => "Alerte inondation, vagues ou submersion",
    }
}

/// L’état du ciel un jour donné
#[derive(Clone, Copy, Debug)]
enum Ciel {
    /// Ciel clair
    Clair,

    /// Ciel nuageux
    Nuageux,

    /// Pluie, avec un entier (non-signé sur 32 bits) indiquant le quantité de pluie prévue (en mm)
    Pluie(f32),

    /// Neige, avec un entier indiquant la hauteur de neige prévue (en cm)
    Neige(f32),

    /// Brouillard
    Brouillard,
}

struct Meteo {
    temperature: f32,
    ciel: Ciel,
    alerte: Option<Alerte>,
}

fn que_porter(temperature: f32) -> &'static str {
    if temperature < -10.0 {
        return "un anorak";
    } else if temperature < 0.0 {
        return "un bon manteau";
    } else if temperature < 15.0 {
        return "un petit pull";
    } else if temperature < 25.0 {
        return "une chemise";
    } else {
        return "un T-shirt";
    }
}

fn decrit_ciel(c: Ciel) -> &'static str {
    match c {
        Ciel::Clair => "il fait beau, youpi!",
        Ciel::Nuageux => "il y a des nuages :-(",
        Ciel::Brouillard => "Il y a du brouillard, on n’y voit rien",
        Ciel::Neige(p) | Ciel::Pluie(p) => {
            if p < 10.0 {
                "Il y a un peu de pluie"
            } else if p < 20.0 {
                "Il pleut beaucoup"
            } else {
                "Il pleut des cordes"
            }
        }
    }
}

fn print_meteo(m: Meteo) {
    println!("La météo d’aujourd’hui");

    let tenue = que_porter(m.temperature);
    println!(
        "Il fera {} degrés, on vous conseille de mettre {tenue}",
        m.temperature
    );

    // question 11: ajouter ici l’affichage de la hauteur de pluie / neige (le cas échéant)

    println!("Ciel: {}", decrit_ciel(m.ciel));

    match m.alerte {
        None => println!("Rien à signaler"),
        Some(a) => println!("{}", affiche_alerte(a)),
    }
}

fn to_farenheit(t_celsius: i32) -> i32 {
    ((t_celsius as f32) * 5.0 / 9.0) as i32
}

// fn meteo_aleatoire() -> Meteo {
//     use rand::Rng;
//     let mut rng = rand::thread_rng();

//     let temperature = rng.gen_range(-10.0..30.0);
//     let ciel = match rng.gen_range(0..5) {
//         0 => Ciel::Clair,
//         1 => Ciel::Nuageux,
//         2 => Ciel::Brouillard,
//         3 => Ciel::Pluie(rng.gen_range(0.0..50.0)),
//         4 => Ciel::Neige(rng.gen_range(0.0..50.0)),
//         _ => unreachable!(),
//     };

//     let alerte = match rng.gen_range(0..3) {
//         0 => None,
//         1 => Some(Alerte::Vent),
//         2 => Some(Alerte::Inondation),
//         _ => unreachable!(),
//     };

//     Meteo {
//         temperature,
//         ciel,
//         alerte,
//     }
// }

fn main() {
    let orleans = [
        Meteo {
            temperature: 12.0,
            ciel: Ciel::Brouillard,
            alerte: Some(Alerte::Vent),
        },
        Meteo {
            temperature: 25.0,
            ciel: Ciel::Clair,
            alerte: None,
        },
        Meteo {
            temperature: 12.0,
            ciel: Ciel::Brouillard,
            alerte: Some(Alerte::Vent),
        },
    ];
    let perpignan = [
        Meteo {
            temperature: 25.0,
            ciel: Ciel::Clair,
            alerte: None,
        },
        Meteo {
            temperature: 25.0,
            ciel: Ciel::Clair,
            alerte: None,
        },
        Meteo {
            temperature: 25.0,
            ciel: Ciel::Pluie(10.0),
            alerte: None,
        },
    ];

    println!("Orléans\n");

    for m in orleans {
        print_meteo(m);
        println!("\n");
    }

    println!("\n");
    println!("Perpignan\n");

    for m in perpignan {
        print_meteo(m);
        println!("\n");
    }
}
