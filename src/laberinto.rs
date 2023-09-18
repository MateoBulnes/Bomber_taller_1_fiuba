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
                let mut casillas_aux = controlar_desvios(
                    &self.desvios,
                    &mut casillas_afectadas,
                    &self.dimension,
                    &bomba,
                );
                //casillas_aux =
                //    controlar_obstaculos(&self.obstaculos, &mut casillas_afectadas, &bomba.tipo);
                casillas_aux =
                    controlar_obstaculos(&self.obstaculos, &mut casillas_aux, &bomba.tipo);
                imprimir_vector_tuplas(
                    &casillas_aux,
                    "CASILLAS FINALES CON LAS QUE SE DANIA".to_string(),
                );
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

pub fn bloquea(tipo_bomba: String, tipo_obstaculo: String) -> bool {
    if tipo_obstaculo == "Pared" {
        return true;
    } else if tipo_bomba == "Normal" {
        return true;
    }
    false
}

pub fn anular_casillas(
    obstaculo: (i32, i32, char),
    casillas_afectadas: &Vec<(i32, i32, char)>,
    tipo_bomba: &String,
    tipo_obstaculo: &String,
) -> Vec<(i32, i32, char)> {
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();

    casillas_anuladas.push(obstaculo);

    match obstaculo.2 {
        'R' => {
            for c in casillas_afectadas {
                if c.1 > obstaculo.1 && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'L' => {
            for c in casillas_afectadas {
                if c.1 < obstaculo.1 && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'U' => {
            for c in casillas_afectadas {
                if c.0 < obstaculo.0 && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'D' => {
            for c in casillas_afectadas {
                if c.0 > obstaculo.0 && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
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
    tipo_bomba: &String,
) -> Vec<(i32, i32, char)> {
    let mut casillas_finales = Vec::new();
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();
    let mut cant_obstaculos = 0;

    //Hago una primera iteracion sobre las casillas afectadas para identificar obstaculos y las casillas a anular
    for obst in obstaculos {
        for casilla in casillas_afectadas {
            if obst.posicion_x == casilla.0 && obst.posicion_y == casilla.1 {
                //encontre un obstaculo
                let casillas_aux =
                    anular_casillas(*casilla, &casillas_afectadas, tipo_bomba, &obst.tipo);
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

fn imprimir_vector_tuplas(vector: &Vec<(i32, i32, char)>, titulo: String) {
    println!("CASILLAS {}", titulo);
    println!("===============================================");
    for (elemento1, elemento2, elemento3) in vector {
        println!("({},{},{})", elemento1, elemento2, elemento3);
    }
    println!("===============================================");
}

fn verificar_agregadas(
    afectadas: &Vec<(i32, i32, char)>,
    finales: &Vec<(i32, i32, char)>,
) -> Vec<(i32, i32, char)> {
    let mut verificadas: Vec<(i32, i32, char)> = Vec::new();

    //imprimir_vector_tuplas(afectadas, "AFECTADAS VERIF".to_string());
    //imprimir_vector_tuplas(finales, "FINALES VERIF".to_string());

    for f in finales {
        if !afectadas.contains(f) {
            verificadas.push(*f);
        }
    }

    imprimir_vector_tuplas(&verificadas, "VERIFICADAS".to_string());

    verificadas
}

fn hay_desvio_en_agregadas(desvio: &Desvio, agregadas: &Vec<(i32, i32, char)>) -> bool {
    for a in agregadas {
        if a.0 == desvio.posicion_x && a.1 == desvio.posicion_y {
            return true;
        }
    }

    false
}

fn actualizar_cant_a_recorrer(desvio: &Desvio, bomba: &Bomba, cant_a_recorrer: &i32) -> i32 {
    let mut recorrido = 0;

    if desvio.posicion_x == bomba.posicion_x {
        recorrido = desvio.posicion_y - bomba.posicion_y;
    } else {
        recorrido = desvio.posicion_x - bomba.posicion_x;
    }

    cant_a_recorrer - recorrido
}

pub fn controlar_desvios(
    desvios: &Vec<Desvio>,
    casillas_afectadas: &Vec<(i32, i32, char)>,
    dim: &i32,
    bomba_det: &Bomba,
) -> Vec<(i32, i32, char)> {
    let mut casillas_finales: Vec<(i32, i32, char)> = Vec::new();
    let mut casillas_agregadas: Vec<(i32, i32, char)> = Vec::new();
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();
    let mut desvios_no_afectados: Vec<&Desvio> = Vec::new();

    let mut cant_desvios = 0;
    let mut iter_desvios = 0;
    let mut cant_a_recorrer = bomba_det.alcance;
    let mut cant_cas_evaluadas = 0;

    imprimir_vector_tuplas(casillas_afectadas, "AFECTADAS".to_string());
    for desv in desvios {
        for casilla in casillas_afectadas {
            if (desv.posicion_x == casilla.0 && desv.posicion_y == casilla.1)
                || (hay_desvio_en_agregadas(desv, &casillas_agregadas))
            {
                cant_a_recorrer = actualizar_cant_a_recorrer(&desv, &bomba_det, &cant_a_recorrer);

                //encontre un desvio
                if iter_desvios == 0 {
                    casillas_finales = desv.desviar(
                        &casilla.2,
                        casillas_afectadas,
                        dim,
                        bomba_det,
                        &mut cant_a_recorrer,
                        &mut casillas_anuladas,
                        &mut casillas_agregadas,
                    );

                    cant_desvios += 1;
                    casillas_agregadas = verificar_agregadas(casillas_afectadas, &casillas_finales);
                    iter_desvios = 1;
                }
            } else{
                cant_cas_evaluadas += 1;

                if cant_cas_evaluadas == casillas_afectadas.len(){
                    desvios_no_afectados.push(desv);
                }
            }
        }
        iter_desvios = 0;
        cant_cas_evaluadas = 0;
    }

    if cant_desvios == 0 {
        return casillas_afectadas.to_vec();
    }

    casillas_finales
}
