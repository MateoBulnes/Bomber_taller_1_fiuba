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

pub fn buscar_enemigos(objeto: &str, enemigos: &mut Vec<Enemigo>, pos_x: i32, pos_y: i32) {
    match objeto {
        "F1" => {
            let nuevo_enemigo = Enemigo::new(1, pos_x, pos_y);
            enemigos.push(nuevo_enemigo);
        }

        "F2" => {
            let nuevo_enemigo = Enemigo::new(2, pos_x, pos_y);
            enemigos.push(nuevo_enemigo);
        }

        "F3" => {
            let nuevo_enemigo = Enemigo::new(3, pos_x, pos_y);
            enemigos.push(nuevo_enemigo);
        }

        _ => {}
    }
}

pub fn crear_enemigos(objetos: &[Vec<&str>], enemigos: &mut Vec<Enemigo>) {
    let mut x = 0;

    for (y, fila) in objetos.iter().enumerate() {
        for casilla in fila {
            buscar_enemigos(casilla, enemigos, x, y as i32);
            x += 1;
        }
        x = 0;
    }
}
