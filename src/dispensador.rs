use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::constants::PORCENTAJE_AVISO_BAJA_CANTIDAD;
use crate::contenedor::Contenedor;
use crate::pedido::Pedido;
use crate::trait_contenedor_cafetera::ContenedorCafetera;

pub struct Dispensador{
    id:u32,
    contenedor_cafe:Arc<RwLock<Contenedor>>,
    contenedor_espuma:Arc<RwLock<Contenedor>>,
    contenedor_agua:Arc<RwLock<Contenedor>>,
    contenedor_cacao:Arc<RwLock<Contenedor>>,
}

impl Dispensador{
    pub fn new(
        id:u32,
        contenedor_cafe:Arc<RwLock<Contenedor>>,
        contenedor_agua:Arc<RwLock<Contenedor>>,
        contenedor_espuma:Arc<RwLock<Contenedor>>,
        contenedor_cacao:Arc<RwLock<Contenedor>>
    )-> Self{
        Dispensador{
            id,
            contenedor_cafe,
            contenedor_espuma,
            contenedor_agua,
            contenedor_cacao
        }
    }
    pub fn producir_bebidas(&mut self, pedidos:Arc<RwLock<Vec<Pedido>>>,consumos:Arc<RwLock<HashMap<String,u32>>>,
                           contador_pedidos: Arc<RwLock<u32>>) {
        loop {
            let mut pedido: Pedido = Pedido {
                agua: 0,
                cafe: 0,
                cacao: 0,
                espuma: 0,
            };
            if let Ok(mut pedidos_guard) = pedidos.write() {
                if let Some(pedido_actual) = pedidos_guard.pop() {
                    pedido = pedido_actual;
                } else {
                    println!("Dispenser #{} no tiene mas pedidos a preparar", self.id);
                    break;
                }
            }

            let mut continuar_pedido = true;
            if pedido.cafe > 0 {
                continuar_pedido = Self::servir_ingrediente(self.contenedor_cafe.clone(), pedido.cafe, "cafe".to_string(), "granos".to_string(), consumos.clone());
            }
            if pedido.espuma > 0 && continuar_pedido {
                continuar_pedido = Self::servir_ingrediente(self.contenedor_espuma.clone(), pedido.espuma, "espuma".to_string(), "leche".to_string(), consumos.clone());
            }
            if pedido.agua > 0 && continuar_pedido {
                continuar_pedido = Self::servir_ingrediente(self.contenedor_agua.clone(), pedido.agua, "agua".to_string(), "red".to_string(), consumos.clone());
            }
            if pedido.cacao > 0 && continuar_pedido {
                continuar_pedido = Self::servir_ingrediente(self.contenedor_cacao.clone(), pedido.cacao, "cacao".to_string(), "".to_string(), consumos.clone());
            }
            if let Ok(mut tot_pedidos) = contador_pedidos.write() {
                *tot_pedidos += 1;
            }
        }
    }
    fn servir_ingrediente(arc_contenedor:Arc<RwLock<Contenedor>>,cantidad:u32, clave_contenedor:String, clave_contenedor_rec:String, arc_consumos:Arc<RwLock<HashMap<String,u32>>>)-> bool{
        let mut exito = true;
        if let Ok(mut contenedor) = arc_contenedor.write() {
            if let Some(consumido) = contenedor.obtener_contenido(cantidad){
                if let Ok(mut consumos) = arc_consumos.write(){
                    *consumos.entry(clave_contenedor).or_insert(0) += consumido;
                }
            }else{
                if let Some(consumo_recarga) = contenedor.recargar(){
                    if let Ok(mut consumos) = arc_consumos.write(){
                        *consumos.entry(clave_contenedor_rec.clone()).or_insert(0) += consumo_recarga;
                        if contenedor.nivel_contenedor_recarga() < PORCENTAJE_AVISO_BAJA_CANTIDAD {
                            println!("Atencion! contenedor {} esta con un nivel de menos de {}%",clave_contenedor_rec,PORCENTAJE_AVISO_BAJA_CANTIDAD);
                        }
                    }
                    if let Some(consumido_post_carga) = contenedor.obtener_contenido(cantidad){
                        if let Ok(mut consumos) = arc_consumos.write(){
                            *consumos.entry(clave_contenedor).or_insert(0) += consumido_post_carga;

                        }
                    }else{
                        exito = false;
                    }
                }else{
                    exito = false;
                }
            }
        }
        exito
    }
}


