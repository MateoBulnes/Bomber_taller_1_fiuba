pub struct Enemigo {
    pub vida: u32,
    pub posicion_x: u32,
    pub posicion_y: u32,
}
impl Enemigo {
    pub fn new(vida_enemigo: u32, pos_x: u32, pos_y: u32) -> Self {
        Self {
            vida: vida_enemigo,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }
}

pub fn buscar_enemigos(objeto: &str, enemigos: &mut Vec<Enemigo>, pos_x: u32, pos_y: u32) {
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

pub fn crear_enemigos(objetos: &Vec<Vec<&str>>, enemigos: &mut Vec<Enemigo>) {
    let mut x = 1;
    let mut y = 1;

    for fila in objetos {
        for casilla in fila {
            buscar_enemigos(casilla, enemigos, x, y);
            y += 1;
        }
        x += 1;
        y = 1;
    }
}
