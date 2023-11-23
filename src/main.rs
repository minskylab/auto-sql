use std::{env::var, error::Error};

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

    #[auto_sql(relation = "fruits")]
    pub cakes: Vec<Cake>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let client = Client::new().await;

    let cake = client
        .create_cake(CreateCakeInput {
            id: 1,
            name: "Hello".to_string(),
            fruits: vec![],
        })
        .await?;

    client
        .create_fruit(
            CreateFruitInputBuilder::default()
                .name("Apple".to_string())
                .build()?,
        )
        .await?;
    println!("{:?}", cake);

    Ok(())
}

struct Client {
    _pool: PgPool,
}

impl Client {
    async fn new() -> Self {
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        Self { _pool: pool }
    }
}
