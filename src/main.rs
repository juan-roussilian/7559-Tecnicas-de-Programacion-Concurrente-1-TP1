mod pedido;
mod pedidos_parser;

use std::fs;
use crate::pedidos_parser::PedidosParser;

fn main() {
    let path = "pedidos_ejemplo.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let parser = PedidosParser{archivo:&data};
    println!("{:?}",parser.obtener_pedidos().unwrap())
}
