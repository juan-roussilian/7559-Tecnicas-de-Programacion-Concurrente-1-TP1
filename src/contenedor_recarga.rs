use std::thread;
use std::time::Duration;
use crate::trait_contenedor_cafetera::ContenedorCafetera;

pub struct ContenedorRecarga{
    cantidad:u32,
    capacidad:u32
}

impl ContenedorRecarga{
    pub fn obtener_max_contenido(&mut self) ->u32{
        // TODO: Ademas debe ser llamado por el hilo "recargador" que estara verificando una condvar
        // de la cantidad que hay en el contenedor.
        let max_cantidad = self.cantidad;
        thread::sleep(Duration::from_millis(max_cantidad as u64));
        self.cantidad = 0;
        max_cantidad
    }
}
impl ContenedorCafetera for ContenedorRecarga{
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