use std::io::{Result, SeekFrom};
use tokio::io::AsyncSeekExt;
use tokio::{fs::File, io::BufReader};

use crate::esf_record::EsfRecord;
use crate::read::{read_int, read_str};
use crate::write_json_file::write_json_file;

pub async fn dump_spells() -> std::io::Result<()> {
    let file = File::open("pub/dat_skills001.epf").await?;
    let mut reader = tokio::io::BufReader::new(file);

    let magic = read_str(&mut reader, 3).await?;
    if magic != "ESF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ESF file",
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
        let record = read_esf_record(&mut reader, i).await?;
        write_json_file(
            format!("dump/spells/{}.json", i),
            serde_json::to_string_pretty(&record).unwrap(),
        )
        .await?;
    }

    Ok(())
}

async fn read_esf_record(reader: &mut BufReader<File>, id: u32) -> Result<EsfRecord> {
    let mut record = EsfRecord::new(id);

    let name_length = read_int(reader, 1).await? as usize;
    let shout_length = read_int(reader, 1).await? as usize;

    record.name = read_str(reader, name_length).await?;
    record.shout = read_str(reader, shout_length).await?;
    record.icon = read_int(reader, 2).await?;
    record.gfx = read_int(reader, 2).await?;
    record.tp_cost = read_int(reader, 2).await?;
    record.sp_cost = read_int(reader, 2).await?;
    record.cast_time = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(2)).await?;

    record.spell_type = read_int(reader, 3).await?;
    record.element = read_int(reader, 1).await?;
    record.element_power = read_int(reader, 2).await?;
    record.target_restrict = read_int(reader, 1).await?;
    record.target_type = read_int(reader, 1).await?;

    reader.seek(SeekFrom::Current(4)).await?;

    record.min_damage = read_int(reader, 2).await?;
    record.max_damage = read_int(reader, 2).await?;
    record.accuracy = read_int(reader, 2).await?;

    reader.seek(SeekFrom::Current(5)).await?;

    record.hp = read_int(reader, 2).await?;

    reader.seek(SeekFrom::Current(15)).await?;

    Ok(record)
}
