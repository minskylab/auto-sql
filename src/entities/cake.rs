// use auto_sql::auto_sql;

// #[auto_sql]

// use sqlx::Postgres;

// use auto_sql_macros::AutoSQL;

// #[derive(AutoSQL)]
// pub struct Cake {
//     pub id: i32,
//     pub name: String,
//     // pub fruits: Vec<Fruit>,
// }

// #[auto_sql]
// pub struct Fruit {
//     pub id: i32,
//     pub name: String,

//     // #[auto_sql(relation = "fruits")]
//     pub cakes: Vec<Cake>,
// }

// pub struct Client {
//     pool: sqlx::Pool<Postgres>,
// }

mod cake_operations {
    // use std::str::FromStr;

    use std::error::Error;

    // use async_graphql::InputObject;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use derive_builder::Builder;
    // use sqlx::Error;
    use uuid::Uuid;

    // use super::{Cake, Client};
    // use chrono::{DateTime, Utc};

    // use derive_builder::Builder;
    // use poem_openapi::Object;
    // use sqlx::Row;
    // use uuid::Uuid;

    // use crate::backend::engine::SDKEngine;
    // use crate::common::commons::SortOrder;
    // use crate::errors::sdk::SDKError;
    // use crate::tasks::task::{Task, TaskPriority, TaskStatus};

    // #[async_trait]
    // pub trait CakeCrudOperations {
    //     async fn create_cake(&self, input: CreateCakeInput) -> Result<Cake, Box<dyn Error>>;
    //     async fn get_cake(&self, id: Uuid) -> Result<Cake, Box<dyn Error>>;
    //     async fn get_cakes(&self, input: Option<GetCakeInput>)
    //         -> Result<Vec<Cake>, Box<dyn Error>>;
    //     async fn update_cake(
    //         &self,
    //         id: Uuid,
    //         input: UpdateCakeInput,
    //     ) -> Result<Cake, Box<dyn Error>>;
    //     async fn delete_cake(&self, id: Uuid) -> Result<Cake, Box<dyn Error>>;
    // }

    // pub enum SortOrder {
    //     Asc,
    //     Desc,
    // }

    #[derive(Default, Builder)]
    #[builder(pattern = "owned")]
    pub struct GetCakeInput {
        #[builder(setter(strip_option), default)]
        pub filter: Option<GetCakesWhere>,

        #[builder(setter(strip_option), default)]
        pub sort_by: Option<String>,
        #[builder(setter(strip_option), default)]
        pub sort_order: Option<SortOrder>,

        #[builder(setter(into, strip_option), default = "Some(100)")]
        pub limit: Option<i32>,
        #[builder(setter(into, strip_option), default = "Some(0)")]
        pub offset: Option<i32>,
    }

    #[derive(Default, Builder)]
    #[builder(pattern = "owned")]
    pub struct CreateCakeInput {
        pub title: String,

        #[builder(setter(strip_option), default)]
        pub description: Option<String>,
        #[builder(setter(strip_option), default)]
        pub due_date: Option<DateTime<Utc>>,
        #[builder(setter(strip_option), default)]
        pub project_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub lead_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub parent_id: Option<Uuid>,
    }

    #[derive(Default, Builder)]
    #[builder(pattern = "owned")]
    pub struct UpdateCakeInput {
        #[builder(setter(strip_option), default)]
        pub title: Option<String>,
        #[builder(setter(strip_option), default)]
        pub description: Option<String>,
        #[builder(setter(strip_option), default)]
        pub due_date: Option<DateTime<Utc>>,
        #[builder(setter(strip_option), default)]
        pub project_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub lead_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub parent_id: Option<Uuid>,
    }

    #[derive(Default, Builder)]
    #[builder(pattern = "owned")]
    pub struct GetCakesWhere {
        #[builder(setter(strip_option), default)]
        pub owner_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub title: Option<String>,
        #[builder(setter(strip_option), default)]
        pub description: Option<String>,
        #[builder(setter(strip_option), default)]
        pub due_date: Option<DateTime<Utc>>,
        #[builder(setter(strip_option), default)]
        pub project_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub lead_id: Option<Uuid>,
        #[builder(setter(strip_option), default)]
        pub parent_id: Option<Uuid>,

        #[builder(setter(strip_option), default)]
        pub _and: Option<Vec<GetCakesWhere>>,
        #[builder(setter(strip_option), default)]
        pub _or: Option<Vec<GetCakesWhere>>,
    }

    impl GetCakesWhere {
        pub fn compile_sql(&self) -> String {
            let mut conditions = Vec::new();

            if let Some(owner_id) = &self.owner_id {
                conditions.push(format!("owner_id = {}", owner_id));
            }

            if let Some(title) = &self.title {
                conditions.push(format!("title = '{}'", title));
            }

            if let Some(description) = &self.description {
                conditions.push(format!("description = '{}'", description));
            }

            if let Some(due_date) = &self.due_date {
                conditions.push(format!("due_date = '{}'", due_date));
            }

            if let Some(project_id) = &self.project_id {
                conditions.push(format!("project_id = {}", project_id));
            }

            if let Some(lead_id) = &self.lead_id {
                conditions.push(format!("lead_id = {}", lead_id));
            }

            if let Some(parent_id) = &self.parent_id {
                conditions.push(format!("parent_id = {}", parent_id));
            }

            if let Some(ands) = &self._and {
                let and_conditions: Vec<String> =
                    ands.iter().map(|and| and.compile_sql()).collect();
                conditions.push(format!("({})", and_conditions.join(" AND ")));
            }

            if let Some(ors) = &self._or {
                let or_conditions: Vec<String> = ors.iter().map(|or| or.compile_sql()).collect();
                conditions.push(format!("({})", or_conditions.join(" OR ")));
            }

            conditions.join(" AND ")
        }
    }

    #[async_trait]
    impl CakeCrudOperations for Client {
        // type Error = Error;

        async fn create_cake(&self, input: CreateCakeInput) -> Result<Cake, Box<dyn Error>> {
            todo!()
        }

        async fn get_cake(&self, id: Uuid) -> Result<Cake, Box<dyn Error>> {
            todo!()
        }

        async fn update_cake(
            &self,
            id: Uuid,
            input: UpdateCakeInput,
        ) -> Result<Cake, Box<dyn Error>> {
            todo!()
        }

        async fn delete_cake(&self, id: Uuid) -> Result<Cake, Box<dyn Error>> {
            todo!()
        }

        async fn get_cakes(
            &self,
            input: Option<GetCakeInput>,
        ) -> Result<Vec<Cake>, Box<dyn Error>> {
            todo!()
        }
    }
}
