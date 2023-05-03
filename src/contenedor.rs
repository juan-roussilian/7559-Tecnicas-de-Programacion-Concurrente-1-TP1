use crate::constants::TIEMPO_EN_OBTENER_UNIDAD_DE_INGREDIENTE;
use crate::contenedor_recarga::ContenedorRecarga;
use crate::trait_contenedor_cafetera::ContenedorCafetera;
use std::thread;
use std::time::Duration;

///Struct que representa un contenedor simple. Solo tiene cantidad, capacidad y opcionalmente un contenedor
/// del cual recargarse
pub struct Contenedor {
    cantidad: u32,
    capacidad: u32,
    contenedor_recarga: Option<ContenedorRecarga>,
}

impl Contenedor {
    /// Metodo constructor.
    ///
    /// Se debe cumplir `cantidad <= capacidad`. De no cumplirse esto, retornara un
    /// `Err<String>`, caso contrario, retorna `Ok<Contenedor>`. Si no tiene un contenedor del cual
    /// recargarse, `contenedor_recarga` debe ser `None`
    pub fn new(
        cantidad: u32,
        capacidad: u32,
        contenedor_recarga: Option<ContenedorRecarga>,
    ) -> Result<Self, String> {
        if cantidad <= capacidad {
            Ok(Contenedor {
                cantidad,
                capacidad,
                contenedor_recarga,
            })
        } else {
            Err("La cantidad con la que se inicializa el contenedor no puede ser mayor a la capacidad".into())
        }
    }
    /// Intenta recargar su contenido hasta el maximo de el contenedor de recarga.
    ///
    /// Llama al metodo `obtener_contenido` de su atributo `contenedor_recarga`. Si este es `None` el metodo
    /// retornara `None`. Si existe un contenedor de recarga, retornara un `Some()` con el monto de la recarga
    pub fn recargar(&mut self) -> Option<u32> {
        if let Some(ref mut contenedor) = self.contenedor_recarga {
            if let Some(cantidad) = contenedor.obtener_contenido(self.capacidad - self.cantidad) {
                self.cantidad += cantidad;
                Some(cantidad)
            } else {
                let recarga = contenedor.obtener_max_contenido();
                self.cantidad += recarga;
                Some(recarga)
            }
        } else {
            None
        }
    }
    /// Devuelve la cantidad de ingredientes del contenedor de recarga sobre su total expresada en un porcentaje entero  si este existe.
    /// Caso contradio devuelve 0.
    pub fn nivel_contenedor_recarga(&self) -> u32 {
        if let Some(ref contenedor) = self.contenedor_recarga {
            contenedor.nivel()
        } else {
            0
        }
    }
}
impl ContenedorCafetera for Contenedor {
    fn obtener_contenido(&mut self, cantidad_obtener: u32) -> Option<u32> {
        //un milisegundo equivale a 1gr o 1ml
        if cantidad_obtener <= self.cantidad {
            thread::sleep(Duration::from_millis(
                (cantidad_obtener * TIEMPO_EN_OBTENER_UNIDAD_DE_INGREDIENTE) as u64,
            ));
            self.cantidad -= cantidad_obtener;
            Some(cantidad_obtener)
        } else {
            None
        }
    }
    fn nivel(&self) -> u32 {
        ((self.cantidad as f32 / self.capacidad as f32) * 100.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contenedor_debe_tener_cantidad_menor_o_igual_a_capacidad() {
        assert!(Contenedor::new(1, 0, None).is_err());
    }

    #[test]
    fn contenedor_con_cantidad_500_obtener_contenido_10_devuelve_10() {
        let mut contenedor =
            Contenedor::new(500, 500, None).expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), Some(10))
    }
    #[test]
    fn contenedor_con_cantidad_2_obtener_contenido_10_devuelve_none() {
        let mut contenedor =
            Contenedor::new(2, 500, None).expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), None)
    }
    #[test]
    fn contenedor_con_cantidad_10_obtener_contenido_10_primero_devuelve_10_luego_none() {
        let mut contenedor =
            Contenedor::new(10, 500, None).expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), Some(10));
        assert_eq!(contenedor.obtener_contenido(10), None)
    }
    #[test]
    fn contenedor_con_cantidad_0_y_capacidad_500_luego_de_recargarse_obtener_contenido_500_devuelve_500(
    ) {
        let contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");

        let mut contenedor = Contenedor::new(0, 500, Some(contenedor_recarga))
            .expect("Fallo la creacion del contenedor");

        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(500), Some(500))
    }

    #[test]
    fn contenedor_pide_recarga_total_y_contenedor_recarga_da_lo_que_tiene_y_queda_vacio() {
        let contenedor_recarga =
            ContenedorRecarga::new(10, 500).expect("Fallo la creacion del contenedor de recarga");

        let mut contenedor = Contenedor::new(0, 500, Some(contenedor_recarga))
            .expect("Fallo la creacion del contenedor");

        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(10), Some(10));
        assert_eq!(contenedor.obtener_contenido(1), None);
        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(1), None)
    }

    #[test]
    fn contenedor_cantidad_250_capacidad_500_nivel_devuelve_50() {
        let contenedor =
            Contenedor::new(250, 500, None).expect("Fallo la creacion del contenedor");

        assert_eq!(contenedor.nivel(), 50)
    }

    #[test]
    fn contenedor_con_contenedor_recarga_con_cantidad_100_y_capacidad_500_nivel_contenedor_recarga_devuelve_20(
    ) {
        let contenedor_recarga =
            ContenedorRecarga::new(100, 500).expect("Fallo la creacion del contenedor de recarga");

        let contenedor = Contenedor::new(250, 500, Some(contenedor_recarga))
            .expect("Fallo la creacion del contenedor");

        assert_eq!(contenedor.nivel_contenedor_recarga(), 20);
    }
}
