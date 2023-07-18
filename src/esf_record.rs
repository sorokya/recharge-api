use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct EsfRecord {
    pub id: u32,
    pub name: String,
    pub shout: String,
    pub icon: u32,
    pub gfx: u32,
    pub tp_cost: u32,
    pub sp_cost: u32,
    pub cast_time: u32,
    #[serde(rename = "type")]
    pub spell_type: u32,
    pub element: u32,
    #[serde(rename = "elementPower")]
    pub element_power: u32,
    #[serde(rename = "targetRestrict")]
    pub target_restrict: u32,
    #[serde(rename = "targetType")]
    pub target_type: u32,
    #[serde(rename = "minDamage")]
    pub min_damage: u32,
    #[serde(rename = "maxDamage")]
    pub max_damage: u32,
    pub accuracy: u32,
    pub hp: u32,
}

impl EsfRecord {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}