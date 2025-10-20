pub mod book_controller {
    use std::collections::HashMap;

    pub async fn get_book_by_id(axum::extract::Query(params): axum::extract::Query<HashMap<String, i32>>) -> String {
        let book_id: Option<&i32> = params.get("id");
        match book_id {
            Some(i) => {
                format!("Get items with query params:,{:?},id={:?}", book_id, i)
            }
            _ => {
                "Get params id error".to_string()
            }
        }
    }
}