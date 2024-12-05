pub use async_io::Async;
use axum::{
    extract::{Form, Path, State},
    response::Html,
    routing::get,
    Router,
};
use macro_rules_attribute::apply;

use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use askama::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const OPTIONS: [&str; 3] = ["vert", "rouge", "rose"];
const OPTIONS2: [&str; 5] = [
    "vert bouteille",
    "rouge carmin",
    "vieux rose",
    "bleu pâle",
    "jaune tournesol",
];

#[cfg(test)]
mod tests;
mod traitements;
mod types;

type Histogramme = types::Histogramme;
type SondageDeroulant = types::SondageDeroulant;
type SondageRange = types::SondageRange;
type Sondage = types::Sondage;

impl Histogramme {
    fn mediane(self) -> Option<usize> {
        traitements::mediane(self)
    }

    pub fn affiche_mediane(self) -> String {
        match traitements::mediane(self) {
            Some(m) => format!("{}", m),
            None => String::from("N/A"),
        }
    }
}

impl SondageDeroulant {
    fn new() -> Self {
        SondageDeroulant {
            options: vec![],
            votes: vec![],
        }
    }

    fn add_option(&mut self, nom_opt: &str) {
        self.options.push(nom_opt.to_string());
        self.votes.push(Default::default());
    }

    fn add_vote(&mut self, reponse: &[Option<usize>]) {
        for (i, s_score) in reponse.iter().enumerate() {
            match s_score {
                None => (),
                Some(s) => self.votes[i].frequences[*s] += 1,
            }
        }
    }

    pub(crate) fn meilleure_option(&self) -> Option<usize> {
        traitements::meilleure_option(self.votes.clone())
    }
}

impl SondageRange {
    pub(crate) fn meilleure_option(&self) -> Option<String> {
        traitements::meilleure_option_range(self.votes.clone())
    }

    pub(crate) fn mediane(&self, option: &str) -> Option<u32> {
        traitements::mediane_vec(&self.votes[option])
    }

    pub(crate) fn decile(&self, option: &str, k: &u32) -> u32 {
        let k = *k;
        let votes = self.votes[option].clone();
        traitements::nombre_dans_intervalle(&votes, 10 * k as u32, 10 * (k + 1) as u32)
    }
}

impl SondageRange {
    fn add_option(&mut self, o: &str) {
        self.votes.insert(o.to_string(), vec![]);
    }

    fn add_vote(&mut self, notes: Vec<(String, String)>) {
        for (o, note) in notes.iter() {
            self.votes
                .get_mut(o)
                .expect("bla")
                .push(note.parse().unwrap())
        }
    }
}

#[derive(Clone, Template)]
#[template(path = "main.html")]
struct AppState {
    sondages: HashMap<String, Sondage>,
}

#[apply(smol_macros::main!)]
async fn main(ex: &Arc<smol_macros::Executor<'_>>) -> io::Result<()> {
    let sondage1 = {
        let mut votes = SondageDeroulant::new();
        for o in ["vert", "rouge", "rose"] {
            votes.add_option(o);
        }
        votes
    };

    let sondage2 = {
        let mut this = SondageRange {
            votes: HashMap::new(),
        };
        for o in OPTIONS2 {
            this.add_option(o)
        }
        this
    };

    let app_state = AppState {
        sondages: HashMap::from([
            ("Couleurs".to_string(), Sondage::Deroulant(sondage1)),
            ("Nuances".to_string(), Sondage::Range(sondage2)),
        ]),
    };
    let app_state = Arc::new(Mutex::new(app_state));

    // Build our application with a route.
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/resultats/:sondage", get(handler_resultats))
        .route(
            "/voter/:sondage",
            get(handler_voter).post(handler_voter_post),
        )
        .with_state(app_state);

    // Create a `smol`-based TCP listener.
    let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 3000)).unwrap();
    println!("listening on {}", listener.get_ref().local_addr().unwrap());

    // Run it using `smol_axum`
    smol_axum::serve(ex.clone(), listener, app).await
}

async fn handler_index(State(a): State<Arc<Mutex<AppState>>>) -> Html<String> {
    let a = a.lock().unwrap();
    Html(a.render().unwrap())
}

#[derive(Template)]
#[template(path = "vote.html")]
struct EnTeteSondage<'a> {
    titre: &'a str,
    options: Vec<String>,
    deroulant: bool,
}

async fn handler_voter(
    Path(titre): Path<String>,
    State(a): State<Arc<Mutex<AppState>>>,
) -> Html<String> {
    let a = a.lock().unwrap();
    let sondage = &a.sondages[&titre];
    let en_tete = match sondage {
        Sondage::Deroulant(d) => {
            let options = &d.options[..];
            EnTeteSondage {
                titre: &titre,
                options: options.to_vec(),
                deroulant: true,
            }
        }
        Sondage::Range(r) => {
            let options: Vec<String> = r.votes.keys().cloned().collect();
            EnTeteSondage {
                titre: &titre,
                options,
                deroulant: false,
            }
        }
    };
    Html(en_tete.render().unwrap())
}

#[derive(Template)]
#[template(path = "reponse.html")]
#[derive(Serialize, Deserialize)]
struct ReponseSondage {
    // titre: String,
    #[serde(default)]
    reponses: HashMap<String, Option<usize>>,
}

async fn handler_voter_post(
    Path(nom_sondage): Path<String>,
    State(a): State<Arc<Mutex<AppState>>>,
    Form(reponses): Form<Vec<(String, String)>>,
) -> Html<String> {
    println!("got {reponses:?}");

    let mut app_state = a.lock().unwrap();
    let s = app_state
        .sondages
        .get_mut(&nom_sondage)
        .expect("no such sondage");
    match s {
        Sondage::Deroulant(d) => {
            let reponses: Vec<Option<usize>> = reponses
                .iter()
                .map(|(_, note)| {
                    if !note.is_empty() {
                        Some(note.parse().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            d.add_vote(&reponses)
        }
        Sondage::Range(r) => r.add_vote(reponses),
    }
    Html("coucou".to_string()) //r.render().unwrap())
}

async fn handler_resultats(
    Path(nom_sondage): Path<String>,
    State(a): State<Arc<Mutex<AppState>>>,
) -> Html<String> {
    let a = a.lock().unwrap();
    let s = a.sondages.get(&nom_sondage).expect("l177: no such sondage");
    match s {
        Sondage::Deroulant(d) => Html(d.render().unwrap()),
        Sondage::Range(r) => Html(r.render().unwrap()),
    }
}
