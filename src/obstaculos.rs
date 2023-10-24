#[derive(PartialEq, Debug)]

pub struct Obstaculo {
    pub tipo: TipoObstaculo,
    pub posicion_x: i32,
    pub posicion_y: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TipoObstaculo {
    Pared,
    Roca
}

impl Obstaculo {
    pub fn new(tipo_obstaculo: TipoObstaculo, pos_x: i32, pos_y: i32) -> Self {
        Self {
            tipo: tipo_obstaculo,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }
}
