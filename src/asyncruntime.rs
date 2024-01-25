use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn write_to_file_async(path: &str, data: &str) -> Result<(), std::io::Error> {
    // Open or create the file asynchronously
    let mut file = File::create(path).await?;

    // Write the data to file asynchronously
    file.write_all(data.as_bytes()).await?;

    Ok(())
}