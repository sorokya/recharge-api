use glob::glob;
use std::io::SeekFrom;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::emf::{Emf, ItemSpawn, NpcSpawn, ResourceSpawn};
use crate::read::{read_int, read_str};
use crate::write_json_file::write_json_file;

pub async fn dump_maps() -> std::io::Result<()> {
    for entry in glob("maps/*.emf").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => dump_map(path).await?,
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

pub async fn dump_map(path: PathBuf) -> std::io::Result<()> {
    let id = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let file = File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);

    let magic = read_str(&mut reader, 3).await?;
    if magic != "EMF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid EMF file",
        ));
    }

    let _rid1 = read_int(&mut reader, 2).await?;
    let _rid2 = read_int(&mut reader, 2).await?;

    let mut map = Emf::new(id);

    let mut name_bytes = [0u8; 32];
    reader.read_exact(&mut name_bytes).await?;

    for i in 0..name_bytes.len() {
        name_bytes[i] += if i % 2 == 0 { 2 } else { 1 };
    }

    let name_bytes: Vec<u8> = name_bytes
        .iter()
        .filter(|b| **b != 0xFF)
        .map(|b| *b)
        .collect();

    map.name = String::from_utf8(name_bytes).unwrap();

    reader.seek(SeekFrom::Current(45)).await?;

    let num_of_npcs = read_int(&mut reader, 1).await?;
    for _ in 0..num_of_npcs {
        let x = read_int(&mut reader, 1).await?;
        let y = read_int(&mut reader, 1).await?;
        let id = read_int(&mut reader, 2).await?;

        reader.seek(SeekFrom::Current(5)).await?;

        let speed = read_int(&mut reader, 1).await?;

        reader.seek(SeekFrom::Current(2)).await?;

        let time = read_int(&mut reader, 1).await?;
        let amount = read_int(&mut reader, 1).await?;

        map.npcs.push(NpcSpawn {
            x,
            y,
            id,
            speed,
            time,
            amount,
        });
    }

    let speed = read_int(&mut reader, 1).await?;

    let num_of_items = read_int(&mut reader, 1).await?;
    for _ in 0..num_of_items {
        let x = read_int(&mut reader, 1).await?;
        let y = read_int(&mut reader, 1).await?;
        let key = read_int(&mut reader, 2).await?;
        let slot = read_int(&mut reader, 1).await?;
        let item_id = read_int(&mut reader, 2).await?;
        let time = read_int(&mut reader, 2).await?;
        let amount = read_int(&mut reader, 3).await?;

        map.items.push(ItemSpawn {
            x,
            y,
            key,
            slot,
            item_id,
            time,
            amount,
        });
    }

    let num_of_resources = read_int(&mut reader, 1).await?;
    for _ in 0..num_of_resources {
        let x = read_int(&mut reader, 1).await?;
        let y = read_int(&mut reader, 1).await?;

        reader.seek(SeekFrom::Current(9)).await?;

        let item_id = read_int(&mut reader, 2).await?;
        let time = read_int(&mut reader, 2).await?;

        reader.seek(SeekFrom::Current(1)).await?;

        let max_amount = read_int(&mut reader, 1).await?;

        map.resources.push(ResourceSpawn {
            x,
            y,
            item_id,
            time,
            max_amount,
        });
    }

    write_json_file(
        format!("dump/maps/{}.json", map.id),
        serde_json::to_string_pretty(&map).unwrap(),
    )
    .await?;

    Ok(())
}
