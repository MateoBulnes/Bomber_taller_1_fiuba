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
    fn get_posicion(&self) -> (u32, u32);
}

impl Objeto for Enemigo {
    fn get_posicion(&self) -> (u32, u32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Obstaculo {
    fn get_posicion(&self) -> (u32, u32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Bomba {
    fn get_posicion(&self) -> (u32, u32) {
        (self.posicion_x.clone(), self.posicion_y.clone())
    }
}
impl Objeto for Desvio {
    fn get_posicion(&self) -> (u32, u32) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_laberinto = &args[1];
    let path_file_salida = &args[2];
    let coord_bomba_x = &args[3];
    let coord_bomba_y = &args[4];

    let base_laberinto: String = leer_laberinto(path_laberinto.to_string());

    let mut objetos: Vec<Vec<&str>> = Vec::new();
    let filas: Vec<&str> = base_laberinto.split('\n').collect(); //Tengo un vector donde cada elemento es una fila del laberinto

    for fila in filas {
        let fila_separada: Vec<&str> = fila.split_whitespace().collect(); //Por cada fila separo por espacios y guardo cada casilla en un vector
        objetos.push(fila_separada); //Agrego la fila a la matriz
    }
    //Construyo el laberinto
    let lab = Laberinto::new(&objetos);

    println!("ENEMIGOS:");
    lab.mostrar_enemigos();
    println!("BOMBAS:");
    lab.mostrar_bombas();
    println!("DESVIOS:");
    lab.mostrar_desvios();
    println!("OBSTACULOS:");
    lab.mostrar_obstaculos();

    println!("=============================================");
    println!(
        "La bomba a detonar esta ubicada en ({}, {})",
        coord_bomba_x, coord_bomba_y
    );
    println!("El archivo de salida se encuentra en {}", path_file_salida);

    match crear_laberinto_resultado(&path_file_salida, &objetos) {
        Ok(_) => println!("Se ha creado el archivo con éxito en {}", path_file_salida),
        Err(err) => eprintln!("Error: {}", err),
    }
}
