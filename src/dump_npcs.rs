use std::io::{Result, SeekFrom};
use tokio::io::AsyncSeekExt;
use tokio::{fs::File, io::BufReader};

use crate::enf_record::EnfRecord;
use crate::read::{read_int, read_prefix_string, read_str};
use crate::write_json_file::write_json_file;

pub async fn dump_npcs() -> std::io::Result<()> {
    let file = File::open("pub/dat_npcs001.epf").await?;
    let mut reader = tokio::io::BufReader::new(file);

    let magic = read_str(&mut reader, 3).await?;
    if magic != "ENF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ENF file",
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
        let record = read_enf_record(&mut reader, i).await?;
        write_json_file(
            format!("dump/npcs/{}.json", i),
            serde_json::to_string_pretty(&record).unwrap(),
        )
        .await?;
    }

    Ok(())
}

async fn read_enf_record(reader: &mut BufReader<File>, id: u32) -> Result<EnfRecord> {
    let mut record = EnfRecord::new(id);
    record.name = read_prefix_string(reader).await?;

    reader.seek(SeekFrom::Current(1)).await?;

    record.graphic = read_int(reader, 2).await?;

    reader.seek(SeekFrom::Current(5)).await?;

    record.npc_type = read_int(reader, 2).await?;
    record.behavior_id = read_int(reader, 2).await?;

    reader.seek(SeekFrom::Current(7)).await?;

    record.hp = read_int(reader, 3).await?;
    record.tp = read_int(reader, 2).await?;
    record.min_damage = read_int(reader, 3).await?;
    record.max_damage = read_int(reader, 3).await?;
    record.hit_rate = read_int(reader, 3).await?;
    record.evasion = read_int(reader, 3).await?;
    record.armor = read_int(reader, 3).await?;
    record.critical_chance = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(7)).await?;

    record.level = read_int(reader, 2).await?;
    record.experience = read_int(reader, 3).await?;

    reader.seek(SeekFrom::Current(1)).await?;

    Ok(record)
}
