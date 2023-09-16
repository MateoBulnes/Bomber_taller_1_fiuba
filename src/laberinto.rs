use crate::bombas::Bomba;
use crate::desvios::Desvio;
use crate::enemigos::Enemigo;
use crate::obstaculos::Obstaculo;

use crate::bombas::crear_bombas;
use crate::desvios::crear_desvios;
use crate::enemigos::crear_enemigos;
use crate::obstaculos::crear_obstaculos;

pub struct Laberinto {
    //cant_filas: i32,
    enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    bombas: Vec<Bomba>,
    desvios: Vec<Desvio>
}

impl Laberinto {
    pub fn new(objetos: &Vec<Vec<&str>>) -> Self {
        Self {
            //cant_filas: contar_filas(&mut objetos),
            enemigos: ubicar_enemigos(objetos),
            obstaculos: ubicar_obstaculos(objetos),
            bombas: ubicar_bombas(objetos),
            desvios: ubicar_desvios(objetos)
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
    }
}

/*fn contar_filas(laberinto: &mut Vec<&str>) -> i32 {
    let lineas: Vec<&str> = laberinto.split('\n').collect();
    lineas.len() as i32
}*/

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
