pub trait ContenedorCafetera {
    fn obtener_contenido(&mut self, cantidad_obtener: u32) -> Option<u32>;
    fn nivel(&self) ->u32;
}
