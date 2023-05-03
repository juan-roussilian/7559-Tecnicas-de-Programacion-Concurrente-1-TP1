use crate::pedido::Pedido;
///Struct que parsea de Json a structs Pedido
pub struct PedidosParser<'a> {
    archivo: &'a str,
}

impl<'a> PedidosParser<'a> {
    ///Constructor, recibe un archivo json como &str y devuelve un struct PedidosParser
    pub fn new(archivo: &'a str) -> Self {
        PedidosParser { archivo }
    }
    /// Realiza el parseo de string literal a una lista de pedidos en forma de `Vec<Pedido>` utilizando
    /// el crate serde.
    ///
    /// Si el archivo json tiene algun error en este formato retornara `Err<String>` caso contrario retornara un
    /// `Ok<Vec<Pedido>>`
    pub fn obtener_pedidos(&self) -> Result<Vec<Pedido>, String> {
        match serde_json::from_str(self.archivo) {
            Ok(pedidos) => Ok(pedidos),
            Err(e) => Err(format!("Error recibido al intentar parsear json: \n {}", e)),
        }
    }
}
