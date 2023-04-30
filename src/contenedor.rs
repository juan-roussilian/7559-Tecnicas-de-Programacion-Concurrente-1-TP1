use std::thread;
use std::time::Duration;
use crate::contenedor_recarga::ContenedorRecarga;
use crate::trait_contenedor_cafetera::ContenedorCafetera;

pub struct Contenedor{
    cantidad:u32,
    capacidad:u32,
    contenedor_recarga:Option<ContenedorRecarga>
}

impl Contenedor{
    pub fn new(cantidad:u32, capacidad:u32, contenedor_recarga:Option<ContenedorRecarga>)->Result<Self,String>{
        if cantidad <= capacidad{
            Ok(Contenedor{
                cantidad,
                capacidad,
                contenedor_recarga
            })
        }else{
            Err("La cantidad con la que se inicializa el contenedor no puede ser mayor a la capacidad".into())
        }
    }
    pub fn recargar(&mut self) -> Option<u32>{
        if let Some(ref mut contenedor) = self.contenedor_recarga{
            if let Some(cantidad) = contenedor.obtener_contenido(self.capacidad){
                self.cantidad += cantidad;
                Some(cantidad)
            }else{
                let recarga = contenedor.obtener_max_contenido();
                self.cantidad += recarga;
                Some(recarga)
            }
        }else{
            None
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
    fn contenedor_debe_tener_cantidad_menor_o_igual_a_capacidad()
    {
        assert!(Contenedor::new(1, 0, None).is_err());
    }
}