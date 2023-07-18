use tokio::{fs::File, io::AsyncWriteExt};

pub async fn write_json_file(path: String, json: String) -> std::io::Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(json.as_bytes()).await?;
    Ok(())
}