use std::path::Path;

use bytes::Bytes;
use s3::Client;
use tokio::{fs::File, io::AsyncWriteExt};

const path_to_data: &str = "~/.sync-bfu/";

struct DataLoader {
    s3_client: Client,
}

impl DataLoader {
    pub fn new(s3_client: Client) -> Self {
        Self { s3_client }
    }

    pub async fn load_folder(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self
            .s3_client
            .objects()
            .list_v2(bucket)
            .prefix(prefix)
            .send()
            .await?;

        Ok(response.common_prefixes)
    }

    pub async fn load_file_content(
        &self,
        bucket: &str,
        key: &str,
        base_path: &str,
    ) -> Result<File, Box<dyn std::error::Error>> {
        let file_path = Path::new(base_path).join(key);

        let response = self.s3_client.objects().get(bucket, key).send().await?;
        let bytes = response.bytes().await?;

        let mut file = File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(file)
    }
}
