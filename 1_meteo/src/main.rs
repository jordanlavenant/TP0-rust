use std::{fs::File, path::Path};

use rand::Rng;
use serde::Deserialize;
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

#[derive(Clone)]
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

fn meteo_aleatoire() -> Meteo {
    let mut rng = rand::thread_rng();
    let temperature: f32 = rng.gen_range(-15.0..40.0);
    let ciel = match rng.gen_range(0..5) {
        0 => Ciel::Brouillard,
        1 => Ciel::Clair,
        2 => {
            let hauteur = rng.gen_range(0.0..20.0);
            Ciel::Neige(hauteur)
        }
        3 => Ciel::Nuageux,
        4 => {
            let hauteur = rng.gen_range(0.0..15.0);
            Ciel::Pluie(hauteur)
        }
        _ => unreachable!(),
    };
    let alerte = if rng.gen_range(0.0..100.0) < 20.0 {
        if rng.gen_range(0.0..100.0) < 50.0 {
            Some(Alerte::Inondation)
        } else {
            Some(Alerte::Vent)
        }
    } else {
        None
    };
    Meteo {
        temperature,
        ciel,
        alerte,
    }
}

fn lit_csv_meteo(path: &Path) -> Vec<Meteo> {
    let csv_file = File::open(path).unwrap();
    let mut reader = csv::Reader::from_reader(csv_file);
    let mut res = vec![];
    for r in reader.records() {
        let r = r.expect("Erreur de format csv");
        let temperature: f32 = r[0].parse().expect("erreur de format numÃ©rique champ 0");
        let ciel = if &r[1] == "Clair" {
            Ciel::Clair
        } else if &r[1] == "Nuageux" {
            Ciel::Nuageux
        } else if &r[1] == "Pluie" {
            let hauteur = r[2].parse().expect("erreur de format numÃ©rique champ 2");
            Ciel::Pluie(hauteur)
        } else if &r[1] == "Neige" {
            let hauteur = r[2].parse().expect("erreur de format numÃ©rique champ 2");
            Ciel::Neige(hauteur)
        } else {
            Ciel::Brouillard
        };
        let alerte: Option<Alerte> = match r[3].parse().ok() {
            None => None,
            Some(0) => Some(Alerte::Vent),
            Some(1) => Some(Alerte::Inondation),
            Some(_) => panic!("Erreur de format dâalerte"),
        };

        let meteo: Meteo = Meteo {
            temperature,
            ciel,
            alerte,
        };
        res.push(meteo);
    }
    res
}

fn mediane(v: Vec<Meteo>) -> f32 {
    let mut temp: Vec<f32> = vec![];
    for m in v {
        temp.push(m.temperature);
    }
    temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = temp.len();
    if len % 2 == 0 {
        (temp[len / 2] + temp[len / 2 - 1]) / 2.0
    } else {
        temp[len / 2]
    }
}

fn moyenne(v: Vec<Meteo>) -> f32 {
    let mut sum = 0.0;
    for m in &v {
        sum += m.temperature;
    }
    sum / v.len() as f32
}

fn main() {
    let malibu: Vec<Meteo> = lit_csv_meteo(Path::new("meteo_malibu.csv"));
    let orleans: Vec<Meteo> = lit_csv_meteo(Path::new("meteo_orleans.csv"));

    for m in malibu.clone() {
        print_meteo(m);
        println!("\n");
    }

    for m in orleans.clone() {
        print_meteo(m);
        println!("\n");
    }

    println!("Mediane Malibu: {}", mediane(malibu.clone()));
    println!("Mediane Orléans: {}", mediane(orleans.clone()));

    println!("Moyenne Malibu: {}", moyenne(malibu));
    println!("Moyenne Orléans: {}", moyenne(orleans));
}
