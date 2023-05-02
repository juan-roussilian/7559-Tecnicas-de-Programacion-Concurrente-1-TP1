mod constants;
mod contenedor;
mod contenedor_recarga;
mod pedido;
mod pedidos_parser;
mod trait_contenedor_cafetera;

use crate::constants::*;
use crate::contenedor::Contenedor;
use crate::contenedor_recarga::ContenedorRecarga;
use crate::pedidos_parser::PedidosParser;
use std::fs;
use std::sync::{Arc, RwLock};

fn main() {
    let path = "pedidos_ejemplo.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let pedidos;

    if let Ok(mut lista_pedidos) = PedidosParser::new(&data).obtener_pedidos() {
        pedidos = Arc::new(RwLock::new(lista_pedidos));
    }
    let contenedor_cafe;
    if let Ok(contenedor_granos_cafe) =
        ContenedorRecarga::new(CAPACIDAD_CAFE_GRANO, CAPACIDAD_CAFE_GRANO)
    {
        match Contenedor::new(
            CAPACIDAD_CAFE_MOLIDO,
            CAPACIDAD_CAFE_MOLIDO,
            contenedor_granos_cafe,
        ) {
            Ok(contenedor_cafe_molido) => {
                contenedor_cafe = Arc::new(RwLock::new(contenedor_cafe_molido))
            }
            Err(e) => {
                println!("{}: Revisar valores archivo de constantes", e);
                return;
            }
        }
    }
    let contenedor_espuma;
    if let Ok(contenedor_leche) = ContenedorRecarga::new(CAPACIDAD_LECHE_FRIA, CAPACIDAD_LECHE_FRIA)
    {
        match Contenedor::new(
            CAPACIDAD_LECHE_ESPUMA,
            CAPACIDAD_LECHE_ESPUMA,
            contenedor_leche,
        ) {
            Ok(contenedor_espuma_leche) => {
                contenedor_espuma = Arc::new(RwLock::new(contenedor_espuma_leche))
            }
            Err(e) => {
                println!("{}: Revisar valores archivo de constantes", e);
                return;
            }
        }
    }
    let contenedor_agua;
    if let Ok(red_agua) = ContenedorRecarga::new(f64::INFINITY as u32, f64::INFINITY as u32) {
        match Contenedor::new(CAPACIDAD_AGUA_CALIENTE, CAPACIDAD_AGUA_CALIENTE, red_agua) {
            Ok(contenedor_agua_caliente) => {
                contenedor_agua = Arc::new(RwLock::new(contenedor_agua_caliente))
            }
            Err(e) => {
                println!("{}: Revisar valores archivo de constantes", e);
                return;
            }
        }
    }
}
