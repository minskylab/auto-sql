use std::{env::var, error::Error};

use async_trait::async_trait;
use auto_sql::commons::{AsSQLArtifacts, Introspective};
use auto_sql_macros::AutoSQL;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, AutoSQL)]
#[auto_sql(client = "Client")]
pub struct Cake {
    pub id: i32,
    pub name: String,
    pub fruits: Vec<Fruit>,
}

#[derive(Debug, AutoSQL)]
pub struct Fruit {
    pub id: i32,
    pub name: String,
    // #[auto_sql(relation = "fruits")]
    // pub cakes: Vec<Cake>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let client = Client::new().await;

    let artifacts = Cake::as_sql_artifacts();

    artifacts.iter().for_each(|artifact| {
        println!("{:?}", artifact);
    });

    // let cake = client
    //     .create_cake(CreateCakeInput {
    //         id: 1,
    //         name: "Hello".to_string(),
    //         fruits: vec![],
    //     })
    //     .await?;

    // client
    //     .create_fruit(
    //         CreateFruitInputBuilder::default()
    //             .name("Apple".to_string())
    //             .build()?,
    //     )
    //     .await?;
    // println!("{:?}", cake);

    Ok(())
}

struct Client {
    pool: PgPool,
}

impl Client {
    async fn new() -> Self {
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        Self { pool }
    }
}

#[async_trait]
impl Introspective for Client {
    async fn introspect(&self) -> Result<(), Box<dyn Error>> {
        let schema = "public";

        let tables = sqlx::query!("SELECT table_name, column_name, is_nullable, data_type FROM INFORMATION_SCHEMA.COLUMNS where table_schema = $1 order by table_name, column_name", schema)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| TableColumnDefinition {
                table_name: row.table_name.clone().unwrap(),
                column_name: row.column_name.clone().unwrap(),
                nullable: match row.is_nullable.clone().unwrap().as_str() {
                    "YES" => true,
                    "NO" => false,
                    _ => panic!("Unexpected value for is_nullable"),
                },
                data_type: row.data_type.clone().unwrap(),
            })
            .collect::<Vec<TableColumnDefinition>>();

        let mut db_context = String::new();

        tables.iter().for_each(|table| {
            db_context.push_str(
                format!(
                    "{}: {} | {} [{}]",
                    table.table_name, table.column_name, table.data_type, table.nullable
                )
                .as_str(),
            );
        });

        Ok(())
    }

    async fn introspect_table(&self, table_name: &str) -> Result<(), Box<dyn Error>> {
        let schema = "public";

        let tables = sqlx::query!("SELECT table_name, column_name, is_nullable, data_type FROM INFORMATION_SCHEMA.COLUMNS where table_schema = $1 and table_name = $2 order by table_name, column_name", schema, table_name)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| TableColumnDefinition {
                table_name: row.table_name.clone().unwrap(),
                column_name: row.column_name.clone().unwrap(),
                nullable: match row.is_nullable.clone().unwrap().as_str() {
                    "YES" => true,
                    "NO" => false,
                    _ => panic!("Unexpected value for is_nullable"),
                },
                data_type: row.data_type.clone().unwrap(),
            })
            .collect::<Vec<TableColumnDefinition>>();

        let mut db_context = String::new();

        tables.iter().for_each(|table| {
            db_context.push_str(
                format!(
                    "{}: {} | {} [{}]",
                    table.table_name, table.column_name, table.data_type, table.nullable
                )
                .as_str(),
            );
        });

        Ok(())
    }
}

pub(crate) struct TableColumnDefinition {
    pub(crate) table_name: String,
    pub(crate) column_name: String,
    pub(crate) nullable: bool,
    pub(crate) data_type: String,
}
