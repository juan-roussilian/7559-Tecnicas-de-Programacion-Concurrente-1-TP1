use crate::constants::*;
use crate::contenedor::Contenedor;
use crate::trait_contenedor_cafetera::ContenedorCafetera;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct ConsoleLogger {
    contenedor_cafe: Arc<RwLock<Contenedor>>,
    contenedor_espuma: Arc<RwLock<Contenedor>>,
    contenedor_agua: Arc<RwLock<Contenedor>>,
    contenedor_cacao: Arc<RwLock<Contenedor>>,
    consumos: Arc<RwLock<HashMap<String, u32>>>,
    contador_pedidos: Arc<RwLock<u32>>,
}

impl ConsoleLogger {
    pub fn new(
        contenedor_cafe: Arc<RwLock<Contenedor>>,
        contenedor_agua: Arc<RwLock<Contenedor>>,
        contenedor_espuma: Arc<RwLock<Contenedor>>,
        contenedor_cacao: Arc<RwLock<Contenedor>>,
        consumos: Arc<RwLock<HashMap<String, u32>>>,
        contador_pedidos: Arc<RwLock<u32>>,
    ) -> Self {
        ConsoleLogger {
            contenedor_cafe,
            contenedor_espuma,
            contenedor_agua,
            contenedor_cacao,
            consumos,
            contador_pedidos,
        }
    }
    pub fn loggear_estadisticas(&self) {
        let mut nivel_granos: u32 = 0;
        let mut nivel_cafe: u32 = 0;
        let mut nivel_agua: u32 = 0;
        let mut nivel_leche: u32 = 0;
        let mut nivel_espuma: u32 = 0;
        let mut nivel_cacao: u32 = 0;
        let mut consumo_granos: u32 = 0;
        let mut consumo_cafe: u32 = 0;
        let mut consumo_red: u32 = 0;
        let mut consumo_agua: u32 = 0;
        let mut consumo_leche: u32 = 0;
        let mut consumo_espuma: u32 = 0;
        let mut consumo_cacao: u32 = 0;
        let mut bebidas_preparadas: u32 = 0;
        loop {
            thread::sleep(Duration::from_millis(MILISEGUNDOS_ENTRE_ESTADISTICAS));
            if let Ok(pedidos_guard) = self.contador_pedidos.read() {
                bebidas_preparadas = *pedidos_guard;
            }
            if let Ok(cafe_guard) = self.contenedor_cafe.read() {
                nivel_cafe = cafe_guard.nivel();
                nivel_granos = cafe_guard.nivel_contenedor_recarga();
            }

            if let Ok(agua_guard) = self.contenedor_agua.read() {
                nivel_agua = agua_guard.nivel();
            }

            if let Ok(leche_guard) = self.contenedor_espuma.read() {
                nivel_espuma = leche_guard.nivel();
                nivel_leche = leche_guard.nivel_contenedor_recarga();
            }

            if let Ok(cacao_guard) = self.contenedor_cacao.read() {
                nivel_cacao = cacao_guard.nivel();
            }
            if let Ok(consumos_guard) = self.consumos.read() {
                if let Some(consumo) = consumos_guard.get("granos") {
                    consumo_granos = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("cafe") {
                    consumo_cafe = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("red") {
                    consumo_red = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("agua") {
                    consumo_agua = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("leche") {
                    consumo_leche = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("espuma") {
                    consumo_espuma = *consumo;
                }
                if let Some(consumo) = consumos_guard.get("cacao") {
                    consumo_cacao = *consumo;
                }
            }

            println!("Niveles de contenedores:");
            println!("Cafe en grano:{}%  Cafe molido:{}% Agua caliente:{}% Leche fria:{}%, Espuma de leche:{}%, Cacao:{}%",
                     nivel_granos, nivel_cafe, nivel_agua, nivel_leche, nivel_espuma, nivel_cacao);
            println!("Ingredientes consumidos:");
            println!("Cafe en grano:{}   Cafe molido:{} Agua de red:{}, Agua caliente:{} Leche fria:{}, Espuma de leche:{}, Cacao:{}",
                     consumo_granos, consumo_cafe, consumo_red, consumo_agua, consumo_leche, consumo_espuma, consumo_cacao);
            println!("Bebidas preparadas: {}\n", bebidas_preparadas)
        }
    }
}
