use crate::Histogramme;
use crate::traitements;

#[test]
fn test_mediane() {
    // Un histogramme vide (toutes les fréquences sont à 0)
    let h_vide: Histogramme = Default::default();
    assert_eq!(traitements::mediane(h_vide), None);

    let h_10_partout = Histogramme { frequences: [10; 7] };
    assert_eq!(traitements::mediane(h_10_partout), Some(3));

    let mut h_trop_fort: Histogramme = Default::default();
    h_trop_fort.frequences[6] = 10;
    assert_eq!(traitements::mediane(h_trop_fort), Some(6));
}

#[test]
fn meilleure_option() {
    let h_vide: Histogramme = Default::default();
    let v1 = vec![h_vide];
    assert_eq!(traitements::meilleure_option(v1), None);
	       
    let h_10_partout = Histogramme { frequences: [10; 7] };
    let mut h_trop_fort: Histogramme = Default::default();
    h_trop_fort.frequences[6] = 10;
    let v2 = vec![h_vide, h_10_partout, h_trop_fort];
    assert_eq!(traitements::meilleure_option(v2), Some(2));
}
