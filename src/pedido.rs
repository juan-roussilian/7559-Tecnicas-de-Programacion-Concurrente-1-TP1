use serde::Deserialize;
///Struct de Pedido. Su unico comportamiento es implementar el trait Deserialize para ser parseado por serde
#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub agua: u32,
    pub cafe: u32,
    pub cacao: u32,
    pub espuma: u32,
}
