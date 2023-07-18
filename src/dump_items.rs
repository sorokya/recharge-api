use std::io::{Result, SeekFrom};
use tokio::io::AsyncSeekExt;
use tokio::{fs::File, io::BufReader};

use crate::eif_record::EifRecord;
use crate::read::{read_int, read_prefix_string, read_str};
use crate::write_json_file::write_json_file;

pub async fn dump_items() -> std::io::Result<()> {
    let file = File::open("pub/dat_items001.epf").await?;
    let mut reader = tokio::io::BufReader::new(file);

    let magic = read_str(&mut reader, 3).await?;
    if magic != "EIF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid EIF file",
        ));
    }

    let _rid1 = read_int(&mut reader, 2).await?;
    let _rid2 = read_int(&mut reader, 2).await?;

    let length = read_int(&mut reader, 2).await?;

    let version = read_int(&mut reader, 1).await?;
    if version != 4 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid version",
        ));
    }

    for i in 1..=length {
        let record = read_eif_record(&mut reader, i).await?;
        write_json_file(
            format!("dump/items/{}.json", i),
            serde_json::to_string_pretty(&record).unwrap(),
        )
        .await?;
    }

    Ok(())
}

async fn read_eif_record(reader: &mut BufReader<File>, id: u32) -> Result<EifRecord> {
    let mut record = EifRecord::new(id);
    record.name = read_prefix_string(reader).await?;

    reader.seek(SeekFrom::Current(1)).await?;

    record.graphic = read_int(reader, 2).await?;
    record.item_type = read_int(reader, 1).await?;

    // reserved. no data follows
    if &record.name == "r" {
        return Ok(record);
    }

    record.item_sub_type = read_int(reader, 1).await?;
    record.special = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(4)).await?;

    record.element = read_int(reader, 1).await?;
    record.hp = read_int(reader, 2).await?;
    record.tp = read_int(reader, 2).await?;
    record.sp = read_int(reader, 2).await?;
    record.min_damage = read_int(reader, 2).await?;
    record.max_damage = read_int(reader, 2).await?;
    record.hit_rate = read_int(reader, 2).await?;
    record.evasion = read_int(reader, 2).await?;
    record.armor = read_int(reader, 2).await?;
    record.critical_chance = read_int(reader, 1).await?;
    record.power = read_int(reader, 1).await?;
    record.accuracy = read_int(reader, 1).await?;
    record.dexterity = read_int(reader, 1).await?;
    record.defense = read_int(reader, 1).await?;
    record.vitality = read_int(reader, 1).await?;
    record.aura = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(6)).await?;

    record.param1 = read_int(reader, 3).await?;
    record.param2 = read_int(reader, 1).await?;
    record.param3 = read_int(reader, 1).await?;
    record.required_level = read_int(reader, 2).await?;
    record.required_class = read_int(reader, 2).await?;
    record.required_power = read_int(reader, 2).await?;
    record.required_accuracy = read_int(reader, 2).await?;
    record.required_dexterity = read_int(reader, 2).await?;
    record.required_defense = read_int(reader, 2).await?;
    record.required_vitality = read_int(reader, 2).await?;
    record.required_aura = read_int(reader, 2).await?;
    record.weight = read_int(reader, 1).await?;
    record.range = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(1)).await?;

    record.size = read_int(reader, 1).await?;
    record.sell_price = read_int(reader, 2).await?;

    Ok(record)
}
