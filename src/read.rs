use tokio::{io::{BufReader, AsyncReadExt}, fs::File};
use std::io::Result;

use crate::decode_int::decode_int;

pub async fn read_int(reader: &mut BufReader<File>, len: usize) -> Result<u32> {
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    Ok(decode_int(&buf))
}

pub async fn read_str(reader: &mut BufReader<File>, len: usize) -> Result<String> {
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    Ok(String::from_utf8(buf).unwrap())
}

pub async fn read_prefix_string(reader: &mut BufReader<File>) -> Result<String> {
    let len = read_int(reader, 1).await? as usize;
    read_str(reader, len).await
}