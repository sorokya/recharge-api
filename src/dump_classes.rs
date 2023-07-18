use std::io::Result;
use tokio::{fs::File, io::BufReader};

use crate::ecf_record::EcfRecord;
use crate::read::{read_int, read_prefix_string, read_str};
use crate::write_json_file::write_json_file;

pub async fn dump_classes() -> std::io::Result<()> {
    let file = File::open("pub/dat_class001.epf").await?;
    let mut reader = tokio::io::BufReader::new(file);

    let magic = read_str(&mut reader, 3).await?;
    if magic != "ECF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ECF file",
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
        let record = read_ecf_record(&mut reader, i).await?;
        write_json_file(
            format!("dump/classes/{}.json", i),
            serde_json::to_string_pretty(&record).unwrap(),
        )
        .await?;
    }

    Ok(())
}

async fn read_ecf_record(reader: &mut BufReader<File>, id: u32) -> Result<EcfRecord> {
    let mut record = EcfRecord::new(id);
    record.name = read_prefix_string(reader).await?;
    record.base = read_int(reader, 1).await?;
    record.class_type = read_int(reader, 1).await?;
    record.power = read_int(reader, 2).await?;
    record.accuracy = read_int(reader, 2).await?;
    record.dexterity = read_int(reader, 2).await?;
    record.defense = read_int(reader, 2).await?;
    record.vitality = read_int(reader, 2).await?;
    record.aura = read_int(reader, 2).await?;
    Ok(record)
}
