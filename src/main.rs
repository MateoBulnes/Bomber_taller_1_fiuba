mod bombas;
mod desvios;
mod enemigos;
mod laberinto;
mod obstaculos;

use std::env;
use std::fs;

use bombas::Bomba;
use desvios::Desvio;
use enemigos::Enemigo;
use laberinto::Laberinto;
use obstaculos::Obstaculo;

use std::fs::File;
use std::io::prelude::*;

trait Objeto {
    fn get_posicion(&self) -> (i32, i32);
}

impl Objeto for Enemigo {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Obstaculo {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Bomba {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Desvio {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}

fn leer_laberinto(path: String) -> String {
    let leido = fs::read_to_string(path);

    match leido {
        Ok(contenido) => {
            return contenido;
        }
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            return e.to_string();
        }
    }
}

fn crear_laberinto_resultado(ruta: &str, laberinto: &Vec<Vec<&str>>) -> std::io::Result<()> {
    // Abre el archivo en modo de escritura, creándolo si no existe o truncándolo si ya existe.
    let mut archivo = File::create(format!("{}/salida.txt", ruta))?;

    let mut fila_salida = String::new();
    // Mensaje que quieres escribir en el archivo.
    for fila in laberinto {
        for casilla in fila {
            fila_salida += casilla;
            fila_salida += " ";
        }
        writeln!(archivo, "{}", fila_salida)?;
        fila_salida.clear();
    }

    Ok(())
}

fn convertir_coordenadas(x: &String, y: &String) -> Result<(i32, i32), std::num::ParseIntError> {
    let x_i32 = x.parse::<i32>()?;
    let y_i32 = y.parse::<i32>()?;
    Ok((x_i32, y_i32))
}

fn actualizar_laberinto<'a>(tablero: &mut Vec<Vec<&'a str>>, lab: &'a Laberinto) {
    for e in &lab.enemigos {
        if e.esta_vivo {
            tablero[(e.posicion_x - 1) as usize][(e.posicion_y - 1) as usize] = e.get_vida();
        } else {
            tablero[(e.posicion_x - 1) as usize][(e.posicion_y - 1) as usize] = "_";
        }
    }

    for b in &lab.bombas {
        if b.detonada {
            tablero[(b.posicion_x - 1) as usize][(b.posicion_y - 1) as usize] = "_";
        }
    }
}

fn main() {
    //Leo los argumentos por linea de comandos
    let args: Vec<String> = env::args().collect();
    let path_laberinto = &args[1];
    let path_file_salida = &args[2];
    let coord_bomba_x = &args[3];
    let coord_bomba_y = &args[4];

    //Convierto las coordenadas para trabajar con i32
    //let coordenadas_bomba = convertir_coordenadas(coord_bomba_x, coord_bomba_y);
    let mut coordenadas_bomba = (0, 0);
    match convertir_coordenadas(coord_bomba_x, coord_bomba_y) {
        Ok(coordenadas) => {
            coordenadas_bomba = coordenadas;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let base_laberinto: String = leer_laberinto(path_laberinto.to_string());

    let mut tablero: Vec<Vec<&str>> = Vec::new();
    let filas: Vec<&str> = base_laberinto.split('\n').collect(); //Tengo un vector donde cada elemento es una fila del laberinto

    for fila in filas {
        let fila_separada: Vec<&str> = fila.split_whitespace().collect(); //Por cada fila separo por espacios y guardo cada casilla en un vector
        tablero.push(fila_separada); //Agrego la fila a la matriz
    }
    //Construyo el laberinto
    let mut lab = Laberinto::new(&tablero);

    lab.mostrar_desvios();
    println!("Antes de detonar:");
    lab.mostrar_enemigos();
    lab.detonar_bomba(coordenadas_bomba);
    println!("Despues de detonar:");
    lab.mostrar_enemigos();

    actualizar_laberinto(&mut tablero, &lab);

    //Creo el archivo de salida
    match crear_laberinto_resultado(&path_file_salida, &tablero) {
        Ok(_) => println!("Se ha creado el archivo con éxito en {}", path_file_salida),
        Err(err) => eprintln!("Error: {}", err),
    }
}
