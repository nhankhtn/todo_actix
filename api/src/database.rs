use sqlx::MySqlPool;

pub async fn database_connection(url: String) -> Result<MySqlPool,sqlx::Error> {
    MySqlPool::connect(&url)
        .await
}