pub mod user_controller {
    use std::collections::HashMap;
    use mysql::{from_row, Pool};
    use mysql::prelude::Queryable;

    #[derive(Debug)]
    pub struct User {
        pub id: u32,
        pub user_name: Option<String>,
        pub age: Option<i32>,
    }

    pub async fn get_user_by_id(axum::extract::Query(params): axum::extract::Query<HashMap<String, i32>>) -> String {
        let user_id: Option<&i32> = params.get("id");
        match user_id {
            Some(i) => {

                let url = "mysql://root:root@localhost:3306/axum_test";
                let pool = Pool::new(url).unwrap(); // 获取连接池
                let mut conn = pool.get_conn().unwrap();// 获取链接

                conn.query_iter("select * from user")
                    .unwrap()
                    .for_each(|row| {
                        let r: (i32, String, i32) = from_row(row.unwrap());
                        println!("{}, {},{}", r.0, r.1, r.2,);
                    });

                format!("Get items with query params:,{:?},id={:?}", user_id, i)
            }
            _ => {
                format!("Get params id error")
            }
        }
    }
}