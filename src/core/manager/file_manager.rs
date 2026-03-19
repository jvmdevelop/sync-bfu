use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::RwLock;

use crate::core::manager::data_manager::DataManager;

pub struct FileManager {
    file_map: RwLock<HashMap<String, PathBuf>>,
    data_loader: DataManager,
}

impl FileManager {
    pub fn new(data_loader: DataManager) -> Self {
        FileManager {
            file_map: RwLock::new(HashMap::new()),
            data_loader,
        }
    }

    pub async fn add_file(&self, file_name: String, path_to_file: PathBuf) {
        self.file_map.write().await.insert(file_name, path_to_file);
    }

    pub async fn remove_file(&self, file_name: &str) -> Option<PathBuf> {
        self.file_map.write().await.remove(file_name)
    }

    pub async fn get_file_path(&self, file_name: &str) -> Option<PathBuf> {
        self.file_map.read().await.get(file_name).cloned()
    }

    pub async fn is_file_managed(&self, file_name: &str) -> bool {
        self.file_map.read().await.contains_key(file_name)
    }

    pub async fn load_file_from_s3(
        &self,
        file_name: &str,
        bucket: &str,
        base_path: &Path,
    ) -> Result<()> {
        let content = self
            .data_loader
            .download_file(bucket, file_name, base_path)
            .await?;

        self.add_file(file_name.to_string(), content.path).await;
        Ok(())
    }

    pub async fn get_managed_files(&self) -> Vec<(String, PathBuf)> {
        self.file_map
            .read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub async fn file_exists_on_disk(&self, file_name: &str) -> Result<bool> {
        if let Some(path) = self.get_file_path(file_name).await {
            Ok(fs::metadata(&path).await.is_ok())
        } else {
            Ok(false)
        }
    }
}
