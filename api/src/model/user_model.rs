// pub mod user_model {
    use sqlx::{FromRow};

    #[derive(Debug,FromRow)]
    pub struct User {
        pub id: u32,
        pub user_name: Option<String>,
        pub age: Option<i32>,
    }
// }