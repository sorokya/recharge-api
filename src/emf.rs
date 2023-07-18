use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Emf {
    pub id: u32,
    pub name: String,
    pub npcs: Vec<NpcSpawn>,
    pub items: Vec<ItemSpawn>,
    pub resources: Vec<ResourceSpawn>,
}

impl Emf {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct NpcSpawn {
    pub x: u32,
    pub y: u32,
    pub id: u32,
    pub speed: u32,
    pub time: u32,
    pub amount: u32,
}

#[derive(Debug, Default, Serialize)]
pub struct ItemSpawn {
    pub x: u32,
    pub y: u32,
    pub key: u32,
    pub slot: u32,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    pub time: u32,
    pub amount: u32
}

#[derive(Debug, Default, Serialize)]
pub struct ResourceSpawn {
    pub x: u32,
    pub y: u32,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    pub time: u32,
    #[serde(rename = "maxAmount")]
    pub max_amount: u32,
}