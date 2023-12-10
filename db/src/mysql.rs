use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use config::config::Config;



// 获取数据库连接
pub async fn mysql_pool() -> Pool<MySql> {
    let config: Config = config::config::read_config();
    let db_uri = format!("mysql://{}:{}@{}:{}/{}", config.db.db_user, config.db.db_pwd, config.db.host, config.db.port, config.db.database);
    print!("db uri: {:#}", db_uri);
    let pool_result = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri)
        .await;
    match pool_result {
        Ok(pool) => {
            pool
        }
        Err(e) => panic!("Error: {:?}", e)
    }
}
