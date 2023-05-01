mod constants;
mod contenedor;
mod contenedor_recarga;
mod pedido;
mod pedidos_parser;
mod trait_contenedor_cafetera;

use crate::pedidos_parser::PedidosParser;
use std::fs;

fn main() {
    let path = "pedidos_ejemplo.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let parser = PedidosParser::new(&data);
    //        PedidosParser{archivo:&data};
    println!("{:?}", parser.obtener_pedidos().unwrap())
    // crear dispensadores
    // dispensadores consumen concurrentemente de los contenedores que "producen"
}
