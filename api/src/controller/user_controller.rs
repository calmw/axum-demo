pub mod user_controller {
    use std::collections::HashMap;
    use crate::model::user_model;

    pub async fn get_user_by_id(axum::extract::Query(params): axum::extract::Query<HashMap<String, i32>>) -> String {
        let user_id: Option<&i32> = params.get("id");
        match user_id {
            Some(i) => {
                let mysql_pool = db::mysql::mysql_pool().await;
                let mut conn = mysql_pool.acquire().await;
                sqlx::query("select * FROM user").execute(&mut conn).await.expect("TODO: panic message");
                let mut rows = sqlx::query_as::< _, user_model::User >("SELECT * FROM user")
                    .fetch_all(&mut conn)
                    .await;

                for row in rows.iter() {
                    println!("{:?}", row);
                }




                format!("Get items with query params:,{:?},id={:?}", user_id, i)
            }
            _ => {
                format!("Get params id error")
            }
        }
    }
}