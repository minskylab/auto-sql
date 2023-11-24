use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Introspective {
    async fn introspect(&self) -> Result<(), Box<dyn Error>>;
    async fn introspect_table(&self, table_name: &str) -> Result<(), Box<dyn Error>>;
}
