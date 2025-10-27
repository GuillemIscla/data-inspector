use crate::{
    domain::{prompt_bucket_uuid, watch::Watch, WatchBuilder}, 
    infra::{data_repository::DataRepository, sample::bucket_data_repository::{BucketDataInput, BucketDataRepository}}
};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone)]
pub struct SampleWatchBuilder {}

pub struct SampleWatch {
    input: BucketDataInput,
    bucket_data_repository: BucketDataRepository,
}

impl SampleWatch {
    pub fn new() -> Self {
        let input = BucketDataInput::new(None);
        let bucket_data_repository = BucketDataRepository {};
        SampleWatch {
            input,
            bucket_data_repository
        }
    }
}


#[async_trait]
impl Watch for SampleWatch {
    fn label(&self) -> String {
        "Sample Watch".to_string()
    }
    
    fn title(&self) -> String {
        "Sample watch".to_string()
    }

    fn input(&self) -> String {
        format!("{}", self.input)
    }

    async fn watch(&self, since_ms:u32) -> Result<String> {
        let count = self.bucket_data_repository.get_count(self.input.clone()).await?;
        let data  = self.bucket_data_repository.get_data(self.input.clone(), since_ms).await?;
        Ok(format!("Total number of rows {count}, \nRecent new rows (last {since_ms} milliseconds): \n{data:?}"))
    }
}

#[async_trait]
impl WatchBuilder for SampleWatchBuilder {
    fn label(&self) -> String {
        "Sample Watch".to_string()
    }

    async fn set_input(&self) -> Box<dyn Watch> {
        let bucket = prompt_bucket_uuid("bucket").await.ok().flatten();
        let input = BucketDataInput::new(bucket);
        let bucket_data_repository = BucketDataRepository {};
        let watch = SampleWatch {
            input,
            bucket_data_repository
        };
        Box::new(watch)
    }

}