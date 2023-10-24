#[derive(Clone, PartialEq, Debug)]

pub struct Bomba {
    pub tipo: TipoBomba,
    pub alcance: i32,
    pub posicion_x: i32,
    pub posicion_y: i32,
    pub detonada: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TipoBomba {
    Normal,
    Traspaso,
}

impl Bomba {
    pub fn new(tipo_bomba: TipoBomba, alcance_bomba: i32, pos_x: i32, pos_y: i32) -> Self {
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
