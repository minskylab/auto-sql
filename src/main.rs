use auto_sql_macros::AutoSQL;

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

fn main() {
    println!("Hello, world!");
}
