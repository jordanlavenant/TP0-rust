use crate::{decrit_ciel, que_porter, to_farenheit, Ciel};

#[test]
fn test_decrit_ciel() {
    let b = decrit_ciel(Ciel::Brouillard);
    assert!(b.contains("brouillard"));
    let n = decrit_ciel(Ciel::Nuageux);
    assert!(n.contains("nuages"));
    assert!(!n.contains("brouillard"));
}

#[test]
fn test_conversion_farenheit() {
    let t: i32 = 0;
    assert_eq!(to_farenheit(t), 0);
    let t: i32 = 100;
    assert_eq!(to_farenheit(t), 55);
    let t: i32 = -40;
    assert_eq!(to_farenheit(t), -22);
}

#[test]
fn test_que_porter() {
    let mut t = 30.0;
    assert_eq!(que_porter(t), "un T-shirt");
    t = 20.0;
    assert_eq!(que_porter(t), "une chemise");
    t = -15.0;
    assert_eq!(que_porter(t), "un anorak");
}
