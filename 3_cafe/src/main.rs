#[derive(Clone, Copy)]
enum Sirop {
    Chocolat,
    Noisette,
    Caramel,
}

#[derive(Clone, Copy)]
enum BaseBoisson {
    Chocolat,
    The,
    Cafe { sirop: Option<Sirop>, lait: bool },
}

#[derive(Clone, Copy)]
struct BoissonChaude {
    base: BaseBoisson,
    sucre: u32,
    bio_equitable: bool,
}

fn afficher_boisson(b: &BoissonChaude) {}

fn contient_lactose(b: &BoissonChaude) -> bool {
    match b.base {
        BaseBoisson::Cafe { lait, .. } => lait,
        _ => false,
    }
}

fn contient_sucre(b: &BoissonChaude) -> bool {
    b.sucre > 0
}

fn prix(b: &BoissonChaude) -> u32 {
    let mut prix = 40;
    if b.bio_equitable {
        prix += 10;
    }
    match b.base {
        BaseBoisson::Cafe { lait, sirop } => {
            if lait {
                prix += 5;
            }
            if let Some(_) = sirop {
                prix += 10;
            }
        }

        _ => {}
    }
    prix
}

fn main() {
    let mon_the_sucre = BoissonChaude {
        base: BaseBoisson::The,
        sucre: 127,
        bio_equitable: false,
    };

    let mon_cafe = BoissonChaude {
        base: BaseBoisson::Cafe {
            sirop: Some(Sirop::Caramel),
            lait: true,
        },
        sucre: 0,
        bio_equitable: false,
    };

    println!("Prix du the sucre: {}", prix(&mon_the_sucre));
    println!("Prix du cafe: {}", prix(&mon_cafe));

    println!(
        "Le the contient-il du sucre? {}",
        contient_sucre(&mon_the_sucre)
    );
    println!(
        "Le cafe contient-il du sucre? {}",
        contient_sucre(&mon_cafe)
    );

    println!(
        "Le the contient-il du lactose? {}",
        contient_lactose(&mon_the_sucre)
    );
    println!(
        "Le cafe contient-il du lactose? {}",
        contient_lactose(&mon_cafe)
    );
}
