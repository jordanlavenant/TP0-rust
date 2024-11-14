use std::collections::HashMap;
use askama::Template;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct Histogramme {
    pub(crate) frequences: [u32; 7],
}

#[derive(Clone, Template)]
#[template(path = "resultats.html")]
pub(crate) struct SondageDeroulant {
    pub(crate) options: Vec<String>,
    pub(crate) votes: Vec<Histogramme>,
}


#[derive(Clone, Template)]
#[template(path = "resultats2.html")]
pub(crate) struct SondageRange {
    pub(crate) votes: HashMap<String, Vec<f64>>,
}

#[derive(Clone)]
pub(crate) enum Sondage {
    Deroulant(SondageDeroulant),
    Range(SondageRange)
}
