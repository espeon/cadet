pub async fn store_cursor(cursor: &str) -> anyhow::Result<()> {
    // get cursor location from env CURSOR_FILE
    let cursor_file = std::env::var("CURSOR_FILE").unwrap_or_else(|_| "cursor.txt".to_string());
    tokio::fs::write(cursor_file, cursor.to_string()).await?;
    Ok(())
}

pub async fn load_cursor() -> Option<String> {
    let cursor_file = std::env::var("CURSOR_FILE").unwrap_or_else(|_| "cursor.txt".to_string());
    tokio::fs::read_to_string(cursor_file).await.ok()
}
