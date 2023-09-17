pub struct Obstaculo {
    pub tipo: String,
    pub posicion_x: i32,
    pub posicion_y: i32,
}

impl Obstaculo {
    pub fn new(tipo_obstaculo: String, pos_x: i32, pos_y: i32) -> Self {
        Self {
            tipo: tipo_obstaculo,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }
}

pub fn buscar_obstaculos(objeto: &str, obstaculos: &mut Vec<Obstaculo>, pos_x: i32, pos_y: i32) {
    match objeto {
        "W" => {
            let nuevo_obstaculo = Obstaculo::new("Pared".to_string(), pos_x, pos_y);
            obstaculos.push(nuevo_obstaculo);
        }

        "R" => {
            let nuevo_obstaculo = Obstaculo::new("Roca".to_string(), pos_x, pos_y);
            obstaculos.push(nuevo_obstaculo);
        }

        _ => {}
    }
}

pub fn crear_obstaculos(objetos: &Vec<Vec<&str>>, obstaculos: &mut Vec<Obstaculo>) {
    let mut x = 1;
    let mut y = 1;

    for fila in objetos {
        for casilla in fila {
            buscar_obstaculos(casilla, obstaculos, x, y);
            y += 1;
        }
        x += 1;
        y = 1;
    }
}
