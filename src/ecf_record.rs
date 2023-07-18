use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct EcfRecord {
    pub id: u32,
    pub name: String,
    pub base: u32,
    #[serde(rename = "type")]
    pub class_type: u32,
    pub power: u32,
    pub accuracy: u32,
    pub dexterity: u32,
    pub defense: u32,
    pub vitality: u32,
    pub aura: u32,
}

impl EcfRecord {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}