#[derive(Clone, PartialEq, Debug)]

pub struct Bomba {
    pub tipo: String,
    pub alcance: i32,
    pub posicion_x: i32,
    pub posicion_y: i32,
    pub detonada: bool,
}

impl Bomba {
    pub fn new(tipo_bomba: String, alcance_bomba: i32, pos_x: i32, pos_y: i32) -> Self {
        Self {
            tipo: tipo_bomba,
            alcance: alcance_bomba,
            posicion_x: pos_x,
            posicion_y: pos_y,
            detonada: false,
        }
    }
    
    pub fn detonar(&mut self, dimension: i32) -> Vec<(i32, i32, char)> {
        let mut casillas_afectadas = Vec::new();
        let mut alcance_aux = self.alcance;

        self.detonada = true;

        while alcance_aux > 0 {
            if alcance_aux >= dimension {
                alcance_aux -= 1;
            } else {
                if self.posicion_x - alcance_aux >= 0 {
                    casillas_afectadas.push((self.posicion_x - alcance_aux, self.posicion_y, 'L'));
                }

                if self.posicion_x + alcance_aux < dimension {
                    casillas_afectadas.push((self.posicion_x + alcance_aux, self.posicion_y, 'R'));
                }
                if self.posicion_y - alcance_aux >= 0 {
                    casillas_afectadas.push((self.posicion_x, self.posicion_y - alcance_aux, 'U'));
                }

                if self.posicion_y + alcance_aux < dimension {
                    casillas_afectadas.push((self.posicion_x, self.posicion_y + alcance_aux, 'D'));
                }

                alcance_aux -= 1;
            }
        }

        casillas_afectadas
    }
}

pub fn buscar_bombas(objeto: &str, bombas: &mut Vec<Bomba>, pos_x: i32, pos_y: i32) {
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

pub fn crear_bombas(objetos: &[Vec<&str>], bombas: &mut Vec<Bomba>) {
    let mut x = 0;

    for (y, fila) in objetos.iter().enumerate() {
        for casilla in fila {
            buscar_bombas(casilla, bombas, x, y as i32);
            x += 1;
        }
        x = 0;
    }
}
