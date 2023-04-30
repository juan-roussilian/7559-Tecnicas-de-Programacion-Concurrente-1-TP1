use crate::pedido::Pedido;
pub struct PedidosParser<'a>{
    pub archivo: &'a str
}

impl PedidosParser<'_> {
    pub fn obtener_pedidos(&self) -> Result<Vec<Pedido>,String> {
        match serde_json::from_str(self.archivo){
            Ok(pedidos) => Ok(pedidos),
            Err(e) => Err(format!("Error recibido al intentar parsear json: \n {}",e.to_string()))
        }
    }
}