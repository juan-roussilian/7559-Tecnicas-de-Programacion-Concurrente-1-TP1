use crate::pedido::Pedido;
pub struct PedidosParser<'a>{
    archivo: &'a str
}

impl <'a>PedidosParser<'a> {
    pub fn new(archivo:&'a str)->Self{
        PedidosParser{
            archivo
        }
    }
    pub fn obtener_pedidos(&self) -> Result<Vec<Pedido>,String> {
        match serde_json::from_str(self.archivo){
            Ok(pedidos) => Ok(pedidos),
            Err(e) => Err(format!("Error recibido al intentar parsear json: \n {}",e.to_string()))
        }
    }
}