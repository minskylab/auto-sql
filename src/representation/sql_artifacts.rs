use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Introspective {
    async fn introspect(&self) -> Result<(), Box<dyn Error>>;
    async fn introspect_table(&self, table_name: &str) -> Result<(), Box<dyn Error>>;
}

pub trait AsSQLArtifacts {
    fn as_sql_artifacts() -> Vec<SQLArtifact>;
}

#[derive(Debug)]
pub struct SQLArtifact {
    pub kind: SQLArtifactKind,
    pub name: String,
    pub sql: String,
}

#[derive(Debug)]
pub enum SQLArtifactKind {
    Scalar,
    Table,
    JoinTable(String, String),
    // View,
    // MaterializedView,
    // Index,
    // Sequence,
    // Type,
    // Extension,
}
