use anyhow::Result;
use chrono::{DateTime, Utc};
use core::fmt;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::infra::data_repository::DataRepository;

#[derive(Clone)]
pub struct BucketDataInput {
    bucket: Option<Uuid>,
}

impl fmt::Display for BucketDataInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.bucket {
            Some(bucket) => write!(f, "bucket: {}", bucket),
            None => write!(f, "bucket: None"),
        }
    }
}

impl BucketDataInput {
    pub fn new(bucket:Option<Uuid>) -> Self {
        BucketDataInput { bucket }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BucketData {
    bucket: Uuid,
    points_reached: i32,
    phrase_used: String,
    created_at: DateTime<Utc>,
}

impl fmt::Display for BucketData{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "bucket: {}, points_reached: {}, phrase_used: {}, created_at: {}", 
            self.bucket, 
            self.points_reached, 
            self.phrase_used, 
            self.created_at
        )
    }
}

pub struct BucketDataRepository {}

impl DataRepository<BucketDataInput> for BucketDataRepository {
    async fn get_count(&self, input:BucketDataInput) -> Result<u64> {
        // let db_url = env::var("DATABASE_URL")
        //     .expect("DATABASE_URL must be set (e.g., in .env)");
        //
        let db_url = "postgres://sample_user:mysecretpassword@localhost:54320/sample_db".to_string();
        // Create a small connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        let query = 
            match input.bucket {
                Some(bucket_defined) =>  
                    sqlx::query_scalar(
                        r#"SELECT COUNT(*) FROM bucket_data where bucket = $1"#
                    ).bind(bucket_defined),
                None => 
                    sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM bucket_data"#),
            };
        let result = query.fetch_one(&pool).await?;
        u64::try_from(result).map_err(|e| anyhow::anyhow!(e))
    }

    async fn get_data(&self, input:BucketDataInput, since_ms:u32) -> Result<String> {
        // let db_url = env::var("DATABASE_URL")
        //     .expect("DATABASE_URL must be set (e.g., in .env)");
        //
        let db_url = "postgres://sample_user:mysecretpassword@localhost:54320/sample_db".to_string();

        // Create a small connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        let since_ms = since_ms as i64;
        let query =
            match input.bucket {
                Some(bucket_defined) => 
                    sqlx::query_as::<_, BucketData>(
                        r#"
                        SELECT bucket, points_reached, phrase_used, created_at
                        FROM bucket_data
                        WHERE bucket = $1 and 
                        created_at >= NOW() - ($2::bigint * INTERVAL '1 millisecond')
                        ORDER BY created_at DESC
                        "#,
                    )
                    .bind(bucket_defined)
                    .bind(since_ms),
                None => 
                    sqlx::query_as::<_, BucketData>(
                        r#"
                        SELECT bucket, points_reached, phrase_used, created_at
                        FROM bucket_data
                        WHERE created_at >= NOW() - ($1::bigint * INTERVAL '1 millisecond')
                        ORDER BY created_at DESC
                        "#
                    )
                    .bind(since_ms)
            };
        let rows = query.fetch_all(&pool).await?;
        let rows_str:String = 
            rows
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join("\n");
        Ok(rows_str)
    }
}