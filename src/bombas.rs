pub struct Bomba {
    pub tipo: String,
    pub alcance: u32,
    pub posicion_x: u32,
    pub posicion_y: u32,
}

impl Bomba {
    pub fn new(tipo_bomba: String, alcance_bomba: u32, pos_x: u32, pos_y: u32) -> Self {
        Self {
            tipo: tipo_bomba,
            alcance: alcance_bomba,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }
}

pub fn buscar_bombas(objeto: &str, bombas: &mut Vec<Bomba>, pos_x: u32, pos_y: u32) {
    match objeto {
        "B1" => {
            let nueva_bomba = Bomba::new("Normal".to_string(), 1, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "B2" => {
            let nueva_bomba = Bomba::new("Normal".to_string(), 2, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "B3" => {
            let nueva_bomba = Bomba::new("Normal".to_string(), 3, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "B4" => {
            let nueva_bomba = Bomba::new("Normal".to_string(), 4, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "B5" => {
            let nueva_bomba = Bomba::new("Normal".to_string(), 5, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "S1" => {
            let nueva_bomba = Bomba::new("Traspaso".to_string(), 1, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "S2" => {
            let nueva_bomba = Bomba::new("Traspaso".to_string(), 2, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "S3" => {
            let nueva_bomba = Bomba::new("Traspaso".to_string(), 3, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "S4" => {
            let nueva_bomba = Bomba::new("Traspaso".to_string(), 4, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        "S5" => {
            let nueva_bomba = Bomba::new("Traspaso".to_string(), 5, pos_x, pos_y);
            bombas.push(nueva_bomba);
        }

        _ => {}
    }
}

pub fn crear_bombas(objetos: &Vec<Vec<&str>>, bombas: &mut Vec<Bomba>) {
    let mut x = 1;
    let mut y = 1;

    for fila in objetos {
        for casilla in fila {
            buscar_bombas(casilla, bombas, x, y);
            y += 1;
        }
        x += 1;
        y = 1;
    }
}
