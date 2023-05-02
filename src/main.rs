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
use std::sync::{Arc, RwLock};
use std::{fs, thread};

fn main() {
    let path = "pedidos_ejemplo.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let _pedidos_preparados = Arc::new(RwLock::new(0));
    let _grano_cafe_consumido = Arc::new(RwLock::new(0));
    let _leche_fria_consumida = Arc::new(RwLock::new(0));
    let _agua_de_red_consumida = Arc::new(RwLock::new(0));
    let _cafe_molido_consumido = Arc::new(RwLock::new(0));
    let _espuma_leche_consumido = Arc::new(RwLock::new(0));
    let _agua_caliente_consumida = Arc::new(RwLock::new(0));
    let _cacao_consumido = Arc::new(RwLock::new(0));

    let pedidos;

    if let Ok(mut lista_pedidos) = PedidosParser::new(&data).obtener_pedidos() {
        pedidos = Arc::new(RwLock::new(lista_pedidos));
    };

    let contenedor_cafe =
        match crear_arc_lock_contenedor(CAPACIDAD_CAFE_MOLIDO, CAPACIDAD_CAFE_GRANO) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_espuma =
        match crear_arc_lock_contenedor(CAPACIDAD_LECHE_ESPUMA, CAPACIDAD_LECHE_FRIA) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_agua =
        match crear_arc_lock_contenedor(CAPACIDAD_AGUA_CALIENTE, f64::INFINITY as u32) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_cacao = match crear_arc_lock_contenedor(CAPACIDAD_CACAO, 0) {
        Ok(contenedor_arc) => contenedor_arc,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    let mut dispensers = vec![];

    for _ in 0..CANTIDAD_DISPENSADORES {
        let lock_contenedor_cafe = contenedor_cafe.clone();
        let lock_contenedor_agua = contenedor_agua.clone();
        let lock_contenedor_espuma = contenedor_espuma.clone();
        let lock_contenedor_cacao = contenedor_cacao.clone();

        dispensers.push(thread::spawn(move || println!("A implementar"))/*dispensador_preparar(contenedores,metricas)*/)
    }
}

pub fn crear_arc_lock_contenedor(
    capacidad_contenedor: u32,
    capacidad_contenedor_rec: u32,
) -> Result<Arc<RwLock<Contenedor>>, String> {
    let contenedor_final;
    if capacidad_contenedor_rec > 0 {
        match ContenedorRecarga::new(capacidad_contenedor_rec, capacidad_contenedor_rec) {
            Ok(contenedor_recarga) => {
                match Contenedor::new(
                    capacidad_contenedor,
                    capacidad_contenedor,
                    Some(contenedor_recarga),
                ) {
                    Ok(contenedor) => {
                        contenedor_final = Arc::new(RwLock::new(contenedor));
                        Ok(contenedor_final)
                    }
                    Err(e) => Err(format!(
                        "[Error]{}: Revisar valores de constantes en constants.rs",
                        e
                    )),
                }
            }
            Err(e) => Err(format!(
                "[Error]{}: Revisar valores de constantes en constants.rs",
                e
            )),
        }
    } else {
        match Contenedor::new(capacidad_contenedor, capacidad_contenedor, None) {
            Ok(contenedor) => {
                contenedor_final = Arc::new(RwLock::new(contenedor));
                Ok(contenedor_final)
            }
            Err(e) => Err(format!(
                "[Error]{}: Revisar valores de constantes en constants.rs",
                e
            )),
        }
    }
}
