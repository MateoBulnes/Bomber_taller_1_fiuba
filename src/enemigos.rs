#[derive(PartialEq, Debug)]

pub struct Enemigo {
    pub vida: i32,
    pub posicion_x: i32,
    pub posicion_y: i32,
    pub esta_vivo: bool,
}
impl Enemigo {
    pub fn new(vida_enemigo: i32, pos_x: i32, pos_y: i32) -> Self {
        Self {
            vida: vida_enemigo,
            posicion_x: pos_x,
            posicion_y: pos_y,
            esta_vivo: true,
        }
    }

    pub fn daniar(&mut self, danio: i32) {
        self.vida -= danio;
        if self.vida <= 0 {
            self.esta_vivo = false;
        }
    }

    pub fn get_vida(&self) -> &str {
        match self.vida {
            1 => "F1",

            2 => "F2",

            3 => "F3",

            _ => "_",
        }
    }
}
