#[derive(Clone, Debug, PartialEq)]

pub struct Desvio {
    pub direccion: DireccionDesvio,
    pub posicion_x: i32,
    pub posicion_y: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DireccionDesvio {
    Derecha,
    Izquierda,
    Arriba,
    Abajo,
    Invalida,
}

impl Desvio {
    pub fn new(direc_desvio: DireccionDesvio, pos_x: i32, pos_y: i32) -> Self {
        Self {
            direccion: direc_desvio,
            posicion_x: pos_x,
            posicion_y: pos_y,
        }
    }

    fn anuladas_por_desvio(
        &self,
        direc_rafaga: &char,
        cas_afectadas: &[(i32, i32, char)],
        dim: &i32,
        casillas_agregadas: &mut [(i32, i32, char)],
    ) -> Vec<(i32, i32, char)> {
        let mut cas_anuladas: Vec<(i32, i32, char)> = Vec::new();
        let mut cas_aux: Vec<(i32, i32, char)> = Vec::new();

        cas_aux.extend_from_slice(cas_afectadas);

        cas_aux.extend_from_slice(casillas_agregadas);

        match *direc_rafaga {
            'R' => {
                for i in (self.posicion_x + 1)..*dim + 1 {
                    for c in &cas_aux {
                        if c.0 == i && c.1 == self.posicion_y {
                            cas_anuladas.push(*c);
                        }
                    }
                }
            }

            'L' => {
                for i in 1..self.posicion_x {
                    for c in &cas_aux {
                        if c.0 == i && c.1 == self.posicion_y {
                            cas_anuladas.push(*c);
                        }
                    }
                }
            }

            'U' => {
                for i in 1..self.posicion_y {
                    for c in &cas_aux {
                        if c.0 == self.posicion_x && c.1 == i {
                            cas_anuladas.push(*c);
                        }
                    }
                }
            }

            'D' => {
                for i in (self.posicion_y + 1)..*dim + 1 {
                    for c in &cas_aux {
                        if c.0 == self.posicion_x && c.1 == i {
                            cas_anuladas.push(*c);
                        }
                    }
                }
            }

            _ => {}
        }

        cas_anuladas
    }

    fn agregar_por_desvio(&self, dim: &i32, cant_a_recorrer: &mut i32) -> Vec<(i32, i32, char)> {
        let mut cas_desviadas: Vec<(i32, i32, char)> = Vec::new();
        let direc_desvio = &self.direccion;

        match direc_desvio {
            DireccionDesvio::Derecha => {
                for i in self.posicion_x + 1..*dim + 1 {
                    if *cant_a_recorrer > 0 {
                        cas_desviadas.push((i, self.posicion_y, 'R'));
                        *cant_a_recorrer -= 1;
                    }
                }
            }

            DireccionDesvio::Izquierda => {
                for i in 1..self.posicion_x + 1 {
                    if *cant_a_recorrer > 0 {
                        cas_desviadas.push((self.posicion_x - i, self.posicion_y, 'L'));
                        *cant_a_recorrer -= 1;
                    }
                }
            }

            DireccionDesvio::Arriba => {
                for i in 1..self.posicion_y + 1 {
                    if *cant_a_recorrer > 0 {
                        cas_desviadas.push((self.posicion_x, self.posicion_y - i, 'U'));
                        *cant_a_recorrer -= 1;
                    }
                }
            }

            DireccionDesvio::Abajo => {
                for i in self.posicion_y + 1..*dim + 1 {
                    if *cant_a_recorrer > 0 {
                        cas_desviadas.push((self.posicion_x, i, 'D'));
                        *cant_a_recorrer -= 1;
                    }
                }
            }

            _ => {}
        }

        cas_desviadas
    }

    pub fn desviar(
        &self,
        direc_rafaga: &char,
        cas_afectadas: &[(i32, i32, char)],
        dim: &i32,
        cant_a_recorrer: &i32,
        casillas_anuladas: &mut Vec<(i32, i32, char)>,
        casillas_agregadas: &mut [(i32, i32, char)],
    ) -> Vec<(i32, i32, char)> {
        let mut nuevas_afectadas: Vec<(i32, i32, char)> = Vec::new();
        let mut final_retorno: Vec<(i32, i32, char)> = Vec::new();

        let aux = self.anuladas_por_desvio(direc_rafaga, cas_afectadas, dim, casillas_agregadas);
        casillas_anuladas.extend_from_slice(&aux);

        let mut cant_a_recorrer_aux = *cant_a_recorrer;
        let mut casillas_desviadas = self.agregar_por_desvio(dim, &mut cant_a_recorrer_aux);

        casillas_desviadas.extend_from_slice(casillas_agregadas);

        nuevas_afectadas.extend_from_slice(&casillas_desviadas);
        nuevas_afectadas.extend_from_slice(cas_afectadas);

        for casilla in &nuevas_afectadas {
            if !casillas_anuladas.contains(casilla) {
                final_retorno.push(*casilla);
            }
        }

        final_retorno
    }

    pub fn get_direccion_simple(&self) -> char {
        match &self.direccion {
            DireccionDesvio::Derecha => 'R',
            DireccionDesvio::Izquierda => 'L',
            DireccionDesvio::Arriba => 'U',
            DireccionDesvio::Abajo => 'D',
            _ => 'X',
        }
    }
}
