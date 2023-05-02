use crate::contenedor_recarga::ContenedorRecarga;
use crate::trait_contenedor_cafetera::ContenedorCafetera;
use std::thread;
use std::time::Duration;

pub struct Contenedor {
    cantidad: u32,
    capacidad: u32,
    contenedor_recarga: ContenedorRecarga,
}

impl Contenedor {
    pub fn new(
        cantidad: u32,
        capacidad: u32,
        contenedor_recarga: ContenedorRecarga,
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
    pub fn recargar(&mut self) -> u32 {
        if let Some(cantidad) = self.contenedor_recarga.obtener_contenido(self.capacidad) {
            self.cantidad += cantidad;
            cantidad
        } else {
            let recarga = self.contenedor_recarga.obtener_max_contenido();
            self.cantidad += recarga;
            recarga
        }
    }
}
impl ContenedorCafetera for Contenedor {
    fn obtener_contenido(&mut self, cantidad_obtener: u32) -> Option<u32> {
        //un milisegundo equivale a 1gr o 1ml
        if cantidad_obtener <= self.cantidad {
            thread::sleep(Duration::from_millis(cantidad_obtener as u64));
            self.cantidad -= cantidad_obtener;
            Some(cantidad_obtener)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contenedor_debe_tener_cantidad_menor_o_igual_a_capacidad() {
        let mut contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");
        assert!(Contenedor::new(1, 0, contenedor_recarga).is_err());
    }

    #[test]
    fn contenedor_con_cantidad_500_obtener_contenido_10_devuelve_10() {
        let mut contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");
        let mut contenedor = Contenedor::new(500, 500, contenedor_recarga)
            .expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), Some(10))
    }
    #[test]
    fn contenedor_con_cantidad_2_obtener_contenido_10_devuelve_none() {
        let mut contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");
        let mut contenedor =
            Contenedor::new(2, 500, contenedor_recarga).expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), None)
    }
    #[test]
    fn contenedor_con_cantidad_10_obtener_contenido_10_primero_devuelve_10_luego_none() {
        let mut contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");
        let mut contenedor =
            Contenedor::new(10, 500, contenedor_recarga).expect("Fallo la creacion del contenedor");
        assert_eq!(contenedor.obtener_contenido(10), Some(10));
        assert_eq!(contenedor.obtener_contenido(10), None)
    }
    #[test]
    fn contenedor_con_cantidad_0_y_capacidad_500_luego_de_recargarse_obtener_contenido_500_devuelve_500(
    ) {
        let mut contenedor_recarga =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor de recarga");
        let mut contenedor =
            Contenedor::new(0, 500, contenedor_recarga).expect("Fallo la creacion del contenedor");
        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(500), Some(500))
    }

    #[test]
    fn contenedor_pide_recarga_total_y_contenedor_recarga_da_lo_que_tiene_y_queda_vacio() {
        let mut contenedor_recarga =
            ContenedorRecarga::new(10, 500).expect("Fallo la creacion del contenedor de recarga");
        let mut contenedor =
            Contenedor::new(0, 500, contenedor_recarga).expect("Fallo la creacion del contenedor");
        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(10), Some(10));
        assert_eq!(contenedor.obtener_contenido(1), None);
        contenedor.recargar();
        assert_eq!(contenedor.obtener_contenido(1), None)
    }
}
