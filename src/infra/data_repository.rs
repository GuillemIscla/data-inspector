use anyhow::Result;
pub trait DataRepository<I> {
    fn get_count(&self, input:I) -> impl std::future::Future<Output = Result<u64>> + Send;
    fn get_data(&self, input:I, since_ms:u32) -> impl std::future::Future<Output = Result<String>> + Send;
}