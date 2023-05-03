/// trait con los metodos que debe implementar un contenedor del programa de la cafetera
pub trait ContenedorCafetera {
    /// Obtiene el contenido especificado en `cantidad_obtener` de si mismo, restandoselo a su propio contenido
    /// y realizando la esepera de tiempo que esta obtencion implica.
    /// Si tiene cantidad suficiente para satisfacer la demanda pedida, retorna un `Some()` con dicha capacidad.
    /// Caso contrario retorna None
    fn obtener_contenido(&mut self, cantidad_obtener: u32) -> Option<u32>;
    /// Devuelve su cantidad de ingredientes sobre el total expresada en un porcentaje entero
    fn nivel(&self) -> u32;
}
