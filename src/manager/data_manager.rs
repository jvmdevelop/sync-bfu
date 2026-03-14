use bytes::Bytes;
use s3::Client;

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

    pub async fn load_file(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let response = self.s3_client.objects().get(bucket, key).send().await?;
        Ok(response.bytes().await?)
    }
}
