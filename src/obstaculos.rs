#[derive(PartialEq, Debug)]

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
