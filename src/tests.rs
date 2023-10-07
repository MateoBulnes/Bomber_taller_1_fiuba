use crate::{
    bombas::Bomba, desvios::Desvio, enemigos::Enemigo, laberinto::daniar_enemigos,
    obstaculos::Obstaculo, Laberinto,
};

//TESTS LABERINTO

#[test]
fn crear_laberinto_con_bomba() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B1", "_"],
        vec!["_", "_", "_"],
        vec!["_", "_", "_"],
    ];
    let lab = Laberinto::new(&tablero);

    let bomba = Bomba::new("Normal".to_string(), 1, 1, 0);
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

    let enemigo = Enemigo::new(3, 1, 1);
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

    let desvio = Desvio::new("Abajo".to_string(), 1, 1);
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

    let pared = Obstaculo::new("Pared".to_string(), 0, 1);
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

    let roca = Obstaculo::new("Roca".to_string(), 2, 1);
    assert_eq!(lab.obstaculos, [roca]);
}

//TESTS ENEMIGOS

#[test]
fn daniar_enemigo_lo_dania() {
    let enemigo = Enemigo::new(3, 1, 1);
    let mut casillas_afectadas: Vec<(i32, i32, char)> = Vec::new();
    let mut enemigos: Vec<Enemigo> = Vec::new();
    enemigos.push(enemigo);
    casillas_afectadas.push((1, 1, 'D'));

    daniar_enemigos(&mut enemigos, &casillas_afectadas);

    assert_eq!(enemigos[0].vida, 2);
}

#[test]
fn matar_enemigo_cambia_estado() {
    let enemigo = Enemigo::new(1, 1, 1);
    let mut casillas_afectadas: Vec<(i32, i32, char)> = Vec::new();
    let mut enemigos: Vec<Enemigo> = Vec::new();
    enemigos.push(enemigo);
    casillas_afectadas.push((1, 1, 'D'));

    daniar_enemigos(&mut enemigos, &casillas_afectadas);

    assert_eq!(enemigos[0].esta_vivo, false);
}

//TESTS BOMBAS
#[test]
fn detonar_bomba_normal() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["F1", "B1", "F1"],
        vec!["F1", "F1", "F1"],
        vec!["F1", "F1", "F1"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert!(lab.bombas[0].detonada);
    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, false);
    assert_eq!(lab.enemigos[2].esta_vivo, true);
    assert_eq!(lab.enemigos[3].esta_vivo, false);
    assert_eq!(lab.enemigos[4].esta_vivo, true);
    assert_eq!(lab.enemigos[5].esta_vivo, true);
    assert_eq!(lab.enemigos[6].esta_vivo, true);
    assert_eq!(lab.enemigos[7].esta_vivo, true);
}

#[test]
fn detonar_bomba_traspaso() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["F1", "S1", "F1"],
        vec!["F1", "F1", "F1"],
        vec!["F1", "F1", "F1"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert!(lab.bombas[0].detonada);
    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, false);
    assert_eq!(lab.enemigos[2].esta_vivo, true);
    assert_eq!(lab.enemigos[3].esta_vivo, false);
    assert_eq!(lab.enemigos[4].esta_vivo, true);
    assert_eq!(lab.enemigos[5].esta_vivo, true);
    assert_eq!(lab.enemigos[6].esta_vivo, true);
    assert_eq!(lab.enemigos[7].esta_vivo, true);
}

#[test]
fn detonar_bombas_en_cadena() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["F1", "F1", "B4", "F1", "F1"],
        vec!["F1", "F1", "F1", "F1", "F1"],
        vec!["F1", "F1", "B2", "F1", "F1"],
        vec!["F1", "F1", "F1", "F1", "F1"],
        vec!["F1", "F1", "F1", "F1", "F1"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((2, 0));

    assert!(lab.bombas[0].detonada);
    assert!(lab.bombas[1].detonada);
    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, false);
    assert_eq!(lab.enemigos[2].esta_vivo, false);
    assert_eq!(lab.enemigos[3].esta_vivo, false);
    assert_eq!(lab.enemigos[6].esta_vivo, false);
    assert_eq!(lab.enemigos[9].esta_vivo, false);
    assert_eq!(lab.enemigos[10].esta_vivo, false);
    assert_eq!(lab.enemigos[11].esta_vivo, false);
    assert_eq!(lab.enemigos[12].esta_vivo, false);
    assert_eq!(lab.enemigos[15].esta_vivo, false);
    assert_eq!(lab.enemigos[20].esta_vivo, false);

    assert_eq!(lab.enemigos[4].esta_vivo, true);
    assert_eq!(lab.enemigos[5].esta_vivo, true);
    assert_eq!(lab.enemigos[7].esta_vivo, true);
    assert_eq!(lab.enemigos[8].esta_vivo, true);
    assert_eq!(lab.enemigos[13].esta_vivo, true);
    assert_eq!(lab.enemigos[14].esta_vivo, true);
    assert_eq!(lab.enemigos[16].esta_vivo, true);
    assert_eq!(lab.enemigos[17].esta_vivo, true);
    assert_eq!(lab.enemigos[18].esta_vivo, true);
    assert_eq!(lab.enemigos[19].esta_vivo, true);
    assert_eq!(lab.enemigos[21].esta_vivo, true);
    assert_eq!(lab.enemigos[22].esta_vivo, true);
}

//TESTS OBSTACULOS
#[test]
fn roca_con_bomba_normal_protege_enemigo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B2", "_"],
        vec!["_", "R", "_"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert!(lab.enemigos[0].esta_vivo);
}

#[test]
fn roca_con_bomba_traspaso_no_protege_enemigo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "S2", "_"],
        vec!["_", "R", "_"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
}

#[test]
fn pared_con_bomba_normal_protege_enemigo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B2", "_"],
        vec!["_", "W", "_"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert!(lab.enemigos[0].esta_vivo);
}

#[test]
fn pared_con_bomba_traspaso_no_protege_enemigo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "S2", "_"],
        vec!["_", "W", "_"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert!(lab.enemigos[0].esta_vivo);
}

//TESTS DESVIOS
#[test]
fn desvio_arriba() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "F1", "_"],
        vec!["B3", "DU", "F1"],
        vec!["_", "_", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((0, 1));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, true);
}

#[test]
fn desvio_abajo() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_"],
        vec!["B3", "DD", "F1"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((0, 1));

    assert_eq!(lab.enemigos[0].esta_vivo, true);
    assert_eq!(lab.enemigos[1].esta_vivo, false);
}

#[test]
fn desvio_izquierda() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B3", "_"],
        vec!["F1", "DL", "_"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, true);
}

#[test]
fn desvio_derecha() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "B3", "_"],
        vec!["_", "DR", "F1"],
        vec!["_", "F1", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((1, 0));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, true);
}

#[test]
fn desvios_arriba_y_derecha() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "DR", "F1", "F1"],
        vec!["_", "_", "F1", "_", "_"],
        vec!["B5", "F1", "DU", "F1", "_"],
        vec!["_", "_", "_", "_", "_"],
        vec!["_", "_", "_", "_", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((0, 2));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, true);
    assert_eq!(lab.enemigos[2].esta_vivo, false);
    assert_eq!(lab.enemigos[3].esta_vivo, false);
    assert_eq!(lab.enemigos[4].esta_vivo, true);
}

#[test]
fn desvios_abajo_e_izquierda() {
    let tablero: Vec<Vec<&str>> = vec![
        vec!["_", "_", "_", "_", "_"],
        vec!["B5", "F1", "DD", "F1", "_"],
        vec!["_", "_", "F1", "_", "_"],
        vec!["F1", "F1", "DL", "_", "_"],
        vec!["_", "_", "F1", "_", "_"],
    ];
    let mut lab = Laberinto::new(&tablero);

    lab.detonar_bomba((0, 1));

    assert_eq!(lab.enemigos[0].esta_vivo, false);
    assert_eq!(lab.enemigos[1].esta_vivo, true);
    assert_eq!(lab.enemigos[2].esta_vivo, false);
    assert_eq!(lab.enemigos[3].esta_vivo, false);
    assert_eq!(lab.enemigos[4].esta_vivo, false);
    assert_eq!(lab.enemigos[5].esta_vivo, true);
}
