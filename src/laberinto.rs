use crate::bombas::Bomba;
use crate::desvios::Desvio;
use crate::enemigos::Enemigo;
use crate::obstaculos::Obstaculo;

use crate::bombas::crear_bombas;
use crate::desvios::crear_desvios;
use crate::enemigos::crear_enemigos;
use crate::obstaculos::crear_obstaculos;

pub struct Laberinto {
    dimension: i32,
    pub enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    pub bombas: Vec<Bomba>,
    desvios: Vec<Desvio>,
}

impl Laberinto {
    pub fn new(objetos: &Vec<Vec<&str>>) -> Self {
        Self {
            dimension: objetos.len() as i32,
            enemigos: ubicar_enemigos(objetos),
            obstaculos: ubicar_obstaculos(objetos),
            bombas: ubicar_bombas(objetos),
            desvios: ubicar_desvios(objetos),
        }
    }

    pub fn detonar_bomba(&mut self, coordenadas_bomba: (i32, i32)) {
        for bomba in &mut self.bombas {
            if bomba.posicion_x == coordenadas_bomba.0 && bomba.posicion_y == coordenadas_bomba.1 {
                let mut casillas_afectadas = bomba.detonar(self.dimension);
                let casillas_aux = controlar_obstaculos(&self.obstaculos, &mut casillas_afectadas);
                daniar_enemigos(&mut self.enemigos, &casillas_aux);
            }
        }
    }

    pub fn mostrar_enemigos(&self) {
        for e in self.enemigos.iter() {
            println!(
                "ENEMIGO con Vida: {}, X: {}, Y: {}",
                e.vida, e.posicion_x, e.posicion_y
            );
        }
    }
    /*
    pub fn mostrar_bombas(&self) {
        for b in self.bombas.iter() {
            println!(
                "BOMBA con Tipo: {}, Alcance: {}, X: {}, Y: {}",
                b.tipo, b.alcance, b.posicion_x, b.posicion_y
            );
        }
    }

    pub fn mostrar_desvios(&self) {
        for d in self.desvios.iter() {
            println!(
                "DESVIO con Tipo: {}, X: {}, Y: {}",
                d.direccion, d.posicion_x, d.posicion_y
            );
        }
    }

    pub fn mostrar_obstaculos(&self) {
        for o in self.obstaculos.iter() {
            println!(
                "OBSTACULO con Tipo: {}, X: {}, Y: {}",
                o.tipo, o.posicion_x, o.posicion_y
            );
        }
    }*/
}

pub fn ubicar_enemigos(laberinto: &Vec<Vec<&str>>) -> Vec<Enemigo> {
    let mut enemigos: Vec<Enemigo> = Vec::new();

    crear_enemigos(laberinto, &mut enemigos);

    enemigos
}

pub fn ubicar_bombas(laberinto: &Vec<Vec<&str>>) -> Vec<Bomba> {
    let mut bombas: Vec<Bomba> = Vec::new();

    crear_bombas(laberinto, &mut bombas);

    bombas
}

pub fn ubicar_desvios(laberinto: &Vec<Vec<&str>>) -> Vec<Desvio> {
    let mut desvios: Vec<Desvio> = Vec::new();

    crear_desvios(laberinto, &mut desvios);

    desvios
}

pub fn ubicar_obstaculos(laberinto: &Vec<Vec<&str>>) -> Vec<Obstaculo> {
    let mut obstaculos: Vec<Obstaculo> = Vec::new();

    crear_obstaculos(laberinto, &mut obstaculos);

    obstaculos
}

pub fn daniar_enemigos(enemigos: &mut Vec<Enemigo>, casillas_afectadas: &Vec<(i32, i32, char)>) {
    for enemigo in enemigos {
        for casilla in casillas_afectadas {
            if enemigo.posicion_x == casilla.0 && enemigo.posicion_y == casilla.1 {
                enemigo.daniar(1);
            }
        }
    }
}

pub fn anular_casillas(
    obstaculo: (i32, i32, char),
    casillas_afectadas: &Vec<(i32, i32, char)>,
) -> Vec<(i32, i32, char)> {
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();

    casillas_anuladas.push(obstaculo);

    match obstaculo.2 {
        'R' => {
            for c in casillas_afectadas {
                if c.1 > obstaculo.1 {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'L' => {
            for c in casillas_afectadas {
                if c.1 < obstaculo.1 {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'U' => {
            for c in casillas_afectadas {
                if c.0 < obstaculo.0 {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'D' => {
            for c in casillas_afectadas {
                if c.0 > obstaculo.0 {
                    casillas_anuladas.push(*c);
                }
            }
        }

        _ => {}
    }

    casillas_anuladas
}

pub fn controlar_obstaculos(
    obstaculos: &Vec<Obstaculo>,
    casillas_afectadas: &Vec<(i32, i32, char)>,
) -> Vec<(i32, i32, char)> {
    let mut casillas_finales = Vec::new();
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();
    let mut cant_obstaculos = 0;

    //Hago una primera iteracion sobre las casillas afectadas para identificar obstaculos y las casillas a anular
    for obst in obstaculos {
        for casilla in casillas_afectadas {
            if obst.posicion_x == casilla.0 && obst.posicion_y == casilla.1 {
                //encontre un obstaculo
                let casillas_aux = anular_casillas(*casilla, &casillas_afectadas);
                casillas_anuladas.extend_from_slice(&casillas_aux);
                cant_obstaculos += 1;
            }
        }
    }

    //Si no hay obstaculos devuelvo el mismo vector que me llego
    if cant_obstaculos == 0 {
        return casillas_afectadas.to_vec();
    } else {
        //Anulo las casillas correspondientes
        for casilla in casillas_afectadas {
            if !casillas_anuladas.contains(casilla) {
                casillas_finales.push(*casilla);
            }
        }
    }

    casillas_finales
}
