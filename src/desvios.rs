pub struct Desvio {
    pub direccion: String,
    pub posicion_x: u32,
    pub posicion_y: u32,
}

impl Desvio {
    pub fn new(direc_desvio: String, pos_x: u32, pos_y: u32) -> Self {
        Self {
            direccion: direc_desvio,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }
}

pub fn buscar_desvios(objeto: &str, desvios: &mut Vec<Desvio>, pos_x: u32, pos_y: u32) {
    match objeto {
        "DU" => {
            let nuevo_desvio = Desvio::new("Arriba".to_string(), pos_x, pos_y);
            desvios.push(nuevo_desvio);
        }

        "DD" => {
            let nuevo_desvio = Desvio::new("Abajo".to_string(), pos_x, pos_y);
            desvios.push(nuevo_desvio);
        }

        "DL" => {
            let nuevo_desvio = Desvio::new("Izquierda".to_string(), pos_x, pos_y);
            desvios.push(nuevo_desvio);
        }

        "DR" => {
            let nuevo_desvio = Desvio::new("Derecha".to_string(), pos_x, pos_y);
            desvios.push(nuevo_desvio);
        }

        _ => {}
    }
}

pub fn crear_desvios(objetos: &Vec<Vec<&str>>, desvios: &mut Vec<Desvio>) {
    let mut x = 1;
    let mut y = 1;

    for fila in objetos {
        for casilla in fila {
            buscar_desvios(casilla, desvios, x, y);
            y += 1;
        }
        x += 1;
        y = 1;
    }
}