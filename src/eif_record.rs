use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct EifRecord {
    pub id: u32,
    pub name: String,
    pub graphic: u32,
    #[serde(rename = "type")]
    pub item_type: u32,
    #[serde(rename = "subType")]
    pub item_sub_type: u32,
    pub special: u32,
    pub element: u32,
    pub hp: u32,
    pub tp: u32,
    pub sp: u32,
    #[serde(rename = "minDamage")]
    pub min_damage: u32,
    #[serde(rename = "maxDamage")]
    pub max_damage: u32,
    #[serde(rename = "hitRate")]
    pub hit_rate: u32,
    pub evasion: u32,
    pub armor: u32,
    #[serde(rename = "criticalChance")]
    pub critical_chance: u32,
    pub power: u32,
    pub accuracy: u32,
    pub dexterity: u32,
    pub defense: u32,
    pub vitality: u32,
    pub aura: u32,
    pub param1: u32,
    pub param2: u32,
    pub param3: u32,
    #[serde(rename = "requiredLevel")]
    pub required_level: u32,
    #[serde(rename = "requiredClass")]
    pub required_class: u32,
    #[serde(rename = "requiredPower")]
    pub required_power: u32,
    #[serde(rename = "requiredAccuracy")]
    pub required_accuracy: u32,
    #[serde(rename = "requiredDexterity")]
    pub required_dexterity: u32,
    #[serde(rename = "requiredDefense")]
    pub required_defense: u32,
    #[serde(rename = "requiredVitality")]
    pub required_vitality: u32,
    #[serde(rename = "requiredAura")]
    pub required_aura: u32,
    pub weight: u32,
    pub range: u32,
    pub size: u32,
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,
}

impl EifRecord {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}