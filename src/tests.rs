use crate::{bombas::Bomba, desvios::Desvio, enemigos::Enemigo, obstaculos::Obstaculo, Laberinto};

#[test]
fn crear_laberinto_con_bomba() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B1", "_"],
        vec!["_", "_", "_"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let bomba = Bomba::new("Normal".to_string(), 1, 1, 2);
    assert_eq!(lab.bombas, [bomba]);
}

#[test]
fn crear_laberinto_con_enemigo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_"],
        vec!["_", "F3", "_"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let enemigo = Enemigo::new(3, 2, 2);
    assert_eq!(lab.enemigos, [enemigo]);
}

#[test]
fn crear_laberinto_con_desvio() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_"],
        vec!["_", "DD", "_"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let desvio = Desvio::new("Abajo".to_string(), 2, 2);
    assert_eq!(lab.desvios, [desvio]);
}

#[test]
fn crear_laberinto_con_obstaculo_pared() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_"],
        vec!["W", "_", "_"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let pared = Obstaculo::new("Pared".to_string(), 2, 1);
    assert_eq!(lab.obstaculos, [pared]);
}

#[test]
fn crear_laberinto_con_obstaculo_roca() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_"],
        vec!["_", "_", "R"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let roca = Obstaculo::new("Roca".to_string(), 2, 3);
    assert_eq!(lab.obstaculos, [roca]);
}
