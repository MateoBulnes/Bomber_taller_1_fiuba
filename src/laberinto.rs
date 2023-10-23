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
    pub obstaculos: Vec<Obstaculo>,
    pub bombas: Vec<Bomba>,
    pub desvios: Vec<Desvio>,
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
        let mut desvios_ignorados: Vec<Desvio> = Vec::new();
        let mut desvios_aux: Vec<Desvio> = Vec::new();
        let mut bomba_detonada: Bomba = Bomba::new("Normal".to_string(), 0, 0, 0);

        self.bombas = ordenar_bombas(&coordenadas_bomba, &mut self.bombas);

        for bomba in &self.bombas {
            if bomba.posicion_x == coordenadas_bomba.0 && bomba.posicion_y == coordenadas_bomba.1 {
                bomba_detonada = bomba.clone();
            }
        }

        if bomba_detonada.alcance > 0 {
            actualizar_original(&bomba_detonada, &mut self.bombas);
        }

        let mut cant_a_recorrer = bomba_detonada.alcance;
        let mut coord_desvio_anterior = (bomba_detonada.posicion_x, bomba_detonada.posicion_y);
        let mut casillas_afectadas = bomba_detonada.detonar(self.dimension);

        let mut casillas_aux = controlar_bombas(
            &mut self.bombas,
            &mut casillas_afectadas,
            &self.dimension,
            &self.obstaculos,
        );

        casillas_aux = controlar_desvios(
            &self.desvios,
            &mut casillas_aux,
            &self.dimension,
            &mut desvios_ignorados,
            &mut cant_a_recorrer,
            &mut coord_desvio_anterior,
        );

        while !desvios_ignorados.is_empty() {
            desvios_aux.extend_from_slice(&desvios_ignorados);
            casillas_aux = controlar_desvios(
                &desvios_aux,
                &mut casillas_aux,
                &self.dimension,
                &mut desvios_ignorados,
                &mut cant_a_recorrer,
                &mut coord_desvio_anterior,
            );
        }

        daniar_enemigos(&mut self.enemigos, &casillas_aux);
    }
}

fn calcular_distancias(bombas: &Vec<Bomba>, coord: &(i32, i32)) -> Vec<f64> {
    let mut distancias: Vec<f64> = Vec::new();

    for b in bombas {
        let dif_x = b.posicion_x - coord.0;
        let dif_y = b.posicion_y - coord.1;
        let dist = (((dif_x * dif_x) + (dif_y * dif_y)) as f64).sqrt();
        distancias.push(dist);
    }

    distancias
}

fn ordenar_bombas(coord_bomba_det: &(i32, i32), bombas: &mut Vec<Bomba>) -> Vec<Bomba> {
    let mut bombas_ordenadas: Vec<Bomba> = Vec::new();
    let distancias = calcular_distancias(bombas, coord_bomba_det);
    let mut indices: Vec<usize> = (0..bombas.len()).collect();

    for i in 0..bombas.len() {
        for j in i + 1..bombas.len() {
            if distancias[indices[i]] > distancias[indices[j]] {
                indices.swap(i, j);
            }
        }
    }

    for &indice in &indices {
        if indice < bombas.len() {
            bombas_ordenadas.push(bombas[indice].clone());
        }
    }

    bombas_ordenadas
}

fn actualizar_original(bomba_referencia: &Bomba, bombas: &mut Vec<Bomba>) {
    for b in bombas {
        if bomba_referencia == b {
            b.detonada = true;
        }
    }
}

pub fn ubicar_enemigos(laberinto: &[Vec<&str>]) -> Vec<Enemigo> {
    let mut enemigos: Vec<Enemigo> = Vec::new();

    crear_enemigos(laberinto, &mut enemigos);

    enemigos
}

pub fn ubicar_bombas(laberinto: &[Vec<&str>]) -> Vec<Bomba> {
    let mut bombas: Vec<Bomba> = Vec::new();

    crear_bombas(laberinto, &mut bombas);

    bombas
}

pub fn ubicar_desvios(laberinto: &[Vec<&str>]) -> Vec<Desvio> {
    let mut desvios: Vec<Desvio> = Vec::new();

    crear_desvios(laberinto, &mut desvios);

    desvios
}

pub fn ubicar_obstaculos(laberinto: &[Vec<&str>]) -> Vec<Obstaculo> {
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
    if tipo_obstaculo == "Pared" || tipo_bomba == "Normal" {
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
                if c.0 > obstaculo.0
                    && c.1 == obstaculo.1
                    && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'L' => {
            for c in casillas_afectadas {
                if c.0 < obstaculo.0
                    && c.1 == obstaculo.1
                    && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'U' => {
            for c in casillas_afectadas {
                if c.1 < obstaculo.1
                    && c.0 == obstaculo.0
                    && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
                {
                    casillas_anuladas.push(*c);
                }
            }
        }

        'D' => {
            for c in casillas_afectadas {
                if c.1 > obstaculo.1
                    && c.0 == obstaculo.0
                    && bloquea(tipo_bomba.to_string(), tipo_obstaculo.to_string())
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

    for obst in obstaculos {
        for casilla in casillas_afectadas {
            if obst.posicion_x == casilla.0 && obst.posicion_y == casilla.1 {
                
                let casillas_aux =
                    anular_casillas(*casilla, casillas_afectadas, tipo_bomba, &obst.tipo);
                casillas_anuladas.extend_from_slice(&casillas_aux);
                cant_obstaculos += 1;
            }
        }
    }
    
    if cant_obstaculos == 0 {
        return casillas_afectadas.to_vec();
    } else {
        for casilla in casillas_afectadas {
            if !casillas_anuladas.contains(casilla) {
                casillas_finales.push(*casilla);
            }
        }
    }

    casillas_finales
}

fn verificar_agregadas(
    afectadas: &[(i32, i32, char)],
    finales: &Vec<(i32, i32, char)>,
) -> Vec<(i32, i32, char)> {
    let mut verificadas: Vec<(i32, i32, char)> = Vec::new();

    for f in finales {
        if !afectadas.contains(f) {
            verificadas.push(*f);
        }
    }

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

fn actualizar_cant_a_recorrer(
    desvio: &Desvio,
    coord_referencia: &(i32, i32),
    cant_a_recorrer: &i32,
) -> i32 {
    let mut recorrido = 0;

    if desvio.posicion_x == coord_referencia.0 {
        recorrido = desvio.posicion_y - coord_referencia.1;
        recorrido = recorrido.abs();
    } else if desvio.posicion_y == coord_referencia.1 {
        recorrido = desvio.posicion_x - coord_referencia.0;
        recorrido = recorrido.abs();
    }

    (cant_a_recorrer - recorrido).abs()
}

fn recorrer_casillas(
    casillas_afectadas: &Vec<(i32, i32, char)>,
    desv: &Desvio,
    cantidades_aux: (&mut i32, &mut i32, &mut i32, &mut i32, &i32),
    desvios_no_afectados: &mut Vec<Desvio>,
    ignorado: &mut bool,
    coord_desv_anterior: &(i32, i32),
    direc_desvio_anterior: &mut char,
) -> Vec<(i32, i32, char)> {
    let mut casillas_finales: Vec<(i32, i32, char)> = Vec::new();
    let mut casillas_agregadas: Vec<(i32, i32, char)> = Vec::new();
    let mut casillas_anuladas: Vec<(i32, i32, char)> = Vec::new();

    let mut direc_rafaga: char;
    for casilla in casillas_afectadas {
        if (desv.posicion_x == casilla.0 && desv.posicion_y == casilla.1)
            || (hay_desvio_en_agregadas(desv, &casillas_agregadas))
        {
            
            if *cantidades_aux.2 == 0 {
                *cantidades_aux.3 =
                    actualizar_cant_a_recorrer(desv, coord_desv_anterior, cantidades_aux.3);

                if desv.posicion_x == casilla.0 && desv.posicion_y == casilla.1 {
                    direc_rafaga = casilla.2;
                } else {
                    direc_rafaga = *direc_desvio_anterior;
                }

                casillas_finales = desv.desviar(
                    &direc_rafaga,
                    casillas_afectadas,
                    cantidades_aux.4,
                    cantidades_aux.3,
                    &mut casillas_anuladas,
                    &mut casillas_agregadas,
                );

                *cantidades_aux.1 += 1;
                casillas_agregadas = verificar_agregadas(casillas_afectadas, &casillas_finales);
                *cantidades_aux.2 = 1;
            }
        } else {
            *cantidades_aux.0 += 1;

            if *cantidades_aux.0 == casillas_afectadas.len() as i32 {
                desvios_no_afectados.push(desv.clone());
                *ignorado = true;
                casillas_finales.extend_from_slice(casillas_afectadas);
            }
        }
    }

    casillas_finales
}

pub fn controlar_desvios(
    desvios: &Vec<Desvio>,
    casillas_afectadas: &mut Vec<(i32, i32, char)>,
    dim: &i32,
    desvios_ignorados: &mut Vec<Desvio>,
    cant_a_recorrer: &mut i32,
    coord_desv_anterior: &mut (i32, i32),
) -> Vec<(i32, i32, char)> {
    let mut desvios_no_afectados: Vec<Desvio> = Vec::new();

    let mut cant_desvios = 0;
    let mut iter_desvios = 0;
    let mut cant_cas_evaluadas = 0;

    let mut direc_desvio_anterior: char = 'X';

    for desv in desvios {
        let mut ignorado = false;

        let cantidades_aux: (&mut i32, &mut i32, &mut i32, &mut i32, &i32) = (
            &mut cant_cas_evaluadas,
            &mut cant_desvios,
            &mut iter_desvios,
            cant_a_recorrer,
            dim,
        );
        *casillas_afectadas = recorrer_casillas(
            casillas_afectadas,
            desv,
            cantidades_aux,
            &mut desvios_no_afectados,
            &mut ignorado,
            coord_desv_anterior,
            &mut direc_desvio_anterior,
        );

        iter_desvios = 0;
        cant_cas_evaluadas = 0;
        if !ignorado {
            *coord_desv_anterior = (desv.posicion_x, desv.posicion_y);
            direc_desvio_anterior = desv.get_direccion_simple();
        }
    }

    if desvios_no_afectados.is_empty() {
        desvios_ignorados.clear();
    } else if *desvios_ignorados != desvios_no_afectados {
        *desvios_ignorados = desvios_no_afectados;
    } else {
        desvios_ignorados.clear();
    }

    casillas_afectadas.to_vec()
}

fn buscar_bombas_afectadas(
    bomba: &mut Bomba,
    casillas_afectadas: &Vec<(i32, i32, char)>,
    dim: &i32,
) -> Vec<(i32, i32, char)> {
    let mut cas_afect_bombas: Vec<(i32, i32, char)> = Vec::new();
    let mut cas_nueva_bomba: Vec<(i32, i32, char)> = Vec::new();

    for c in casillas_afectadas {
        if c.0 == bomba.posicion_x && c.1 == bomba.posicion_y && !bomba.detonada {
            cas_nueva_bomba = bomba.detonar(*dim);
        }
    }

    cas_afect_bombas.extend_from_slice(casillas_afectadas);

    if !cas_nueva_bomba.is_empty() {
        for cas in cas_nueva_bomba {
            cas_afect_bombas.push(cas);
        }
    }

    cas_afect_bombas
}

pub fn controlar_bombas(
    bombas: &mut Vec<Bomba>,
    casillas_afectadas: &mut Vec<(i32, i32, char)>,
    dimension: &i32,
    obstaculos: &Vec<Obstaculo>,
) -> Vec<(i32, i32, char)> {
    let mut casillas_finales: Vec<(i32, i32, char)> = Vec::new();

    let mut iteracion = 0;

    for b in bombas {
        iteracion += 1;
        if iteracion >= 1 {
            *casillas_afectadas = buscar_bombas_afectadas(b, casillas_afectadas, dimension);

            *casillas_afectadas = controlar_obstaculos(obstaculos, casillas_afectadas, &b.tipo);
        }
    }
    casillas_finales.extend_from_slice(casillas_afectadas);

    casillas_finales
}
