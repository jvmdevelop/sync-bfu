use crate::core::{
    config::Config,
    error::Result,
    manager::{data_manager::DataManager, file_manager::FileManager},
};

mod core;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    tokio::fs::create_dir_all(&config.data_dir).await?;
    println!("Data directory: {:?}", config.data_dir);

    let s3_client = config.create_s3_client()?;
    let data_loader = DataManager::new(s3_client);
    let _file_manager = FileManager::new(data_loader);

    println!("sync-bfu successfully initialized");
    println!("Ready to sync and start surfing: {}", config.bucket);

    Ok(())
}
