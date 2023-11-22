// use auto_sql::auto_sql;

// #[auto_sql]

// #[derive(AutoSQL)]
pub struct Cake {
    pub id: i32,
    pub name: String,

    pub fruits: Vec<Fruit>,
}

// #[auto_sql]
pub struct Fruit {
    pub id: i32,
    pub name: String,

    // #[auto_sql(relation = "fruits")]
    pub cakes: Vec<Cake>,
}
