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
        (self.posicion_x, self.posicion_y)
    }
}
impl Objeto for Obstaculo {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x, self.posicion_y)
    }
}
impl Objeto for Bomba {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x, self.posicion_y)
    }
}
impl Objeto for Desvio {
    fn get_posicion(&self) -> (i32, i32) {
        (self.posicion_x, self.posicion_y)
    }
}

fn leer_laberinto(path: String) -> String {
    let leido = fs::read_to_string(path);

    match leido {
        Ok(contenido) => contenido,
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            e.to_string()
        }
    }
}

fn formatear_ruta_salida(ruta: &str) -> String {
    let mut ruta = ruta.trim_end_matches('/');
    ruta = ruta.trim_end_matches('/');
    format!("./{ruta}/")
}

fn crear_directorio_salida(ruta: &str) -> Result<(), String> {
    if std::path::Path::new(ruta).exists() {
        return Ok(());
    }

    match std::fs::create_dir_all(ruta.clone()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error creando el directorio".into()),
    }
}

fn crear_laberinto_resultado(
    ruta_entrada: &str,
    ruta_salida: &str,
    laberinto: &Vec<Vec<&str>>,
) -> std::io::Result<()> {
    let path_dir = formatear_ruta_salida(ruta_salida);

    match crear_directorio_salida(&path_dir) {
        Ok(_) => (),
        Err(_) => println!("Error creando el directorio"),
    }

    let mut archivo = File::create(format!("{}/{}", ruta_salida, ruta_entrada))?;

    let mut fila_salida = String::new();
    for fila in laberinto {
        for casilla in fila {
            fila_salida += casilla;
            fila_salida += " ";
        }
        if !fila.is_empty() {
            writeln!(archivo, "{}", fila_salida)?;
        }
        fila_salida.clear();
    }

    Ok(())
}

fn actualizar_laberinto<'a>(tablero: &mut [Vec<&'a str>], lab: &'a Laberinto) {
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
    let path_dir = formatear_ruta_salida(ruta_salida);

    match crear_directorio_salida(&path_dir) {
        Ok(_) => (),
        Err(_) => println!("Error creando el directorio"),
    }
    let mut archivo = File::create(format!("{}/{}", ruta_salida, ruta_entrada))?;

    writeln!(archivo, "ERROR: {}", mensaje_error)?;

    Ok(())
}

fn contar_filas(filas: &Vec<&str>) -> usize {
    let mut cant_filas = 0;

    for f in filas {
        if !f.is_empty() {
            cant_filas += 1;
        }
    }

    cant_filas
}

fn main() {
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
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Error: No se pudo crear el archivo de salida.  {}", err)
                    }
                };
            } else {
                let mut tablero: Vec<Vec<&str>> = Vec::new();
                let filas: Vec<&str> = base_laberinto.split('\n').collect();
                let cant_filas = contar_filas(&filas);
                let mut error_dimension = false;

                for fila in filas {
                    let fila_separada: Vec<&str> = fila.split_whitespace().collect();
                    if !fila_separada.is_empty() && fila_separada.len() != (cant_filas) {
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
                        Ok(_) => (),
                        Err(err) => eprintln!("Error: No se pudo crear el archivo de salida.  {}", err),
                    };
                } else {
                    let mut lab = Laberinto::new(&tablero);
                    lab.detonar_bomba(coordenadas_bomba);
                    actualizar_laberinto(&mut tablero, &lab);

                    match crear_laberinto_resultado(&path_laberinto, &path_file_salida, &tablero) {
                        Ok(_) => (),
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
