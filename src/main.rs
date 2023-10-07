mod bombas;
mod desvios;
mod enemigos;
mod laberinto;
mod obstaculos;

#[cfg(test)]
mod tests;

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

fn crear_laberinto_resultado(
    ruta_entrada: &str,
    ruta_salida: &str,
    laberinto: &Vec<Vec<&str>>,
) -> std::io::Result<()> {
    // Abre el archivo en modo de escritura, creándolo si no existe o truncándolo si ya existe.
    let mut archivo = File::create(format!("{}/{}", ruta_salida, ruta_entrada))?;

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

fn actualizar_laberinto<'a>(tablero: &mut Vec<Vec<&'a str>>, lab: &'a Laberinto) {
    for e in &lab.enemigos {
        if e.esta_vivo {
            tablero[(e.posicion_y) as usize][(e.posicion_x) as usize] = e.get_vida();
        } else {
            tablero[(e.posicion_y) as usize][(e.posicion_x) as usize] = "_";
        }
    }

    for b in &lab.bombas {
        if b.detonada {
            tablero[(b.posicion_y) as usize][(b.posicion_x) as usize] = "_";
        }
    }
}

fn validar_input(args: &Vec<String>) -> Result<(String, String, i32, i32), String> {
    if args.len() < 4 {
        return Err(format!("La cantidad de argumentos ingresados no es correcta. Ingresó {} y deben ser 4 argumentos", args.len()));
    }
    let coord_x = match args[3].parse::<i32>() {
        Ok(value) => value,
        Err(err) => {
            return Err(format!(
                "La coordenada X ingresada no es de tipo numérico: {}",
                err
            ))
        }
    };

    let coord_y = match args[4].parse::<i32>() {
        Ok(value) => value,
        Err(err) => {
            return Err(format!(
                "La coordenada Y ingresada no es de tipo numérico: {}",
                err
            ))
        }
    };

    Ok((args[1].clone(), args[2].clone(), coord_x, coord_y))
}

fn crear_archivo_error(
    ruta_entrada: &str,
    ruta_salida: &str,
    mensaje_error: &str,
) -> std::io::Result<()> {
    let mut archivo = File::create(format!("{}/{}", ruta_salida, ruta_entrada))?;

    writeln!(archivo, "ERROR: {}", mensaje_error)?;

    Ok(())
}

/*verificar_dimensiones(tablero: &Vec<Vec<&str>>) {

}*/

fn main() {
    //Leo los argumentos por linea de comandos
    let args: Vec<String> = env::args().collect();

    match validar_input(&args) {
        Ok((path_lab, path_salida, x, y)) => {
            let path_laberinto = path_lab;
            let path_file_salida = path_salida;
            let coordenadas_bomba = (x, y);

            let base_laberinto: String = leer_laberinto(path_laberinto.to_string());

            if base_laberinto.is_empty() {
                match crear_archivo_error(
                    &path_laberinto,
                    &path_file_salida,
                    "El archivo ingresado esta vacío",
                ) {
                    Ok(_) => println!("Se ha creado el archivo con error en {}", path_file_salida),
                    Err(err) => eprintln!("Error: No se pudo crear el archivo de salida.  {}", err),
                };
            } else {
                let mut tablero: Vec<Vec<&str>> = Vec::new();
                let filas: Vec<&str> = base_laberinto.split('\n').collect();
                let cant_filas = filas.len();
                let mut error_dimension = false;

                for fila in filas {
                    let fila_separada: Vec<&str> = fila.split_whitespace().collect();
                    if fila_separada.len() > 0 && fila_separada.len() != (cant_filas - 1) {
                        error_dimension = true;
                        break;
                    }
                    tablero.push(fila_separada);
                }

                if error_dimension {
                    match crear_archivo_error(
                        &path_laberinto,
                        &path_file_salida,
                        "Las dimensiones del tablero del archivo ingresado no son correctas. La cantidad de filas y columnas debe ser la misma",
                    ) {
                        Ok(_) => println!("Se ha creado el archivo con error en {}", path_file_salida),
                        Err(err) => eprintln!("Error: No se pudo crear el archivo de salida.  {}", err),
                    };
                } else {
                    //Construyo el laberinto
                    let mut lab = Laberinto::new(&tablero);
                    lab.detonar_bomba(coordenadas_bomba);
                    actualizar_laberinto(&mut tablero, &lab);

                    //Creo el archivo de salida
                    match crear_laberinto_resultado(&path_laberinto, &path_file_salida, &tablero) {
                        Ok(_) => {
                            println!("Se ha creado el archivo con éxito en {}", path_file_salida)
                        }
                        Err(err) => {
                            eprintln!("Error: No se pudo crear el archivo de salida.  {}", err)
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
