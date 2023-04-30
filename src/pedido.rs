use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub agua: u32,
    pub cafe: u32,
    pub cacao: u32,
    pub espuma: u32
}
