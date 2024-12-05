use std::collections::HashMap;

use crate::Histogramme;

/// Renvoie la valeur médiane d’un histogramme (le plus petit indice `i` tel que la somme
/// des valeurs avant `i` représente au moins la moitié de la somme de tout l’histogramme.)
pub fn mediane(h: Histogramme) -> Option<usize> {
    let total: u32 = h.frequences.iter().sum();
    if total == 0 {
        return None;
    }
    let mut nb = 0;
    for (i, v) in h.frequences.iter().enumerate() {
        nb += v;
        if nb >= (total as f64 / 2.0).ceil() as u32 {
            return Some(i);
        }
    }
    return None;
}

/// Renvoie l’indice de l’histogramme avec la meilleure médiane
pub fn meilleure_option(sondage: Vec<Histogramme>) -> Option<usize> {
    let mut meilleur_mediane: Option<usize> = None;
    let mut meilleur_indice: Option<usize> = None;

    for (i, h) in sondage.iter().enumerate() {
        let mediane = h.mediane();
        match (mediane, meilleur_mediane) {
            (Some(n), None) => {
                meilleur_mediane = Some(n);
                meilleur_indice = Some(i);
            }
            (Some(n), Some(old)) => {
                if n > old {
                    meilleur_mediane = Some(n);
                    meilleur_indice = Some(i);
                }
            }
            (None, _) => (),
        }
    }
    return meilleur_indice;
}

// Ci-dessous, partie sondages "Range" avec slider, à traiter en semaine 2

/// Renvoie la médiane d’un vecteur de flottants
pub fn mediane_vec(v: &[u32]) -> Option<u32> {
    todo!()
}

/// Renvoie la clé associée à la valeur avec la meilleure médiane.
pub fn meilleure_option_range(sondage: HashMap<String, Vec<u32>>) -> Option<String> {
    todo!()
}

/// indique combien de valeurs dans le tableau v sont comprises entre les bornes
pub fn nombre_dans_intervalle(sondage: &[u32], borne_inf: u32, borne_sup: u32) -> u32 {
    todo!()
}
