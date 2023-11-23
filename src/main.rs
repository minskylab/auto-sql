use std::env::var;

use auto_sql_macros::AutoSQL;
use sqlx::{postgres::PgPoolOptions, Connection, Row};
// use std::slice::SliceIndex;

#[derive(AutoSQL)]
pub struct Cake {
    pub id: i32,
    pub name: String,

    pub fruits: Vec<Fruit>,
}

#[derive(AutoSQL)]
pub struct Fruit {
    pub id: i32,
    pub name: String,

    #[auto_sql(relation = "fruits")]
    pub cakes: Vec<Cake>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv::dotenv().ok();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();

    let schema = "public";

    let tables = sqlx::query!("SELECT table_name, column_name, is_nullable, data_type FROM INFORMATION_SCHEMA.COLUMNS where table_schema = $1 order by table_name, column_name", schema).fetch_all(&pool).await.unwrap().iter().map(|row| TableColumnDefinition {
        table_name: row.table_name.clone().unwrap(),
        column_name: row.column_name.clone().unwrap(),
        nullable: match row.is_nullable.clone().unwrap().as_str() {
            "YES" => true,
            "NO" => false,
            _ => panic!("Unexpected value for is_nullable"),
        },
        data_type: row.data_type.clone().unwrap(),
    }).collect::<Vec<TableColumnDefinition>>();

    tables.iter().for_each(|table| {
        println!(
            "{}: {} | {} [{}]",
            table.table_name, table.column_name, table.data_type, table.nullable
        );
    });
}

pub(crate) struct TableColumnDefinition {
    pub(crate) table_name: String,
    pub(crate) column_name: String,
    pub(crate) nullable: bool,
    pub(crate) data_type: String,
}
