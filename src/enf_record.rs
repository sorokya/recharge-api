use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct EnfRecord {
    pub id: u32,
    pub name: String,
    pub graphic: u32,
    #[serde(rename = "type")]
    pub npc_type: u32,
    #[serde(rename = "behaviorId")]
    pub behavior_id: u32,
    pub hp: u32,
    pub tp: u32,
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
    pub level: u32,
    pub experience: u32,
}

impl EnfRecord {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}