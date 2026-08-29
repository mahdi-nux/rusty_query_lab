use sqlx::{SqlitePool, AssertSqlSafe};

pub async fn init(address: String) -> Result<SqlitePool, sqlx::Error> {
    let format = format!("sqlite://{}", address);
    let my_pool = SqlitePool::connect(&format).await?;

    println!("DATABASE URL: {}", format);
    Ok(my_pool)
}

pub async fn run_query(
    connection: SqlitePool,
    user_query: String,
    mode: bool
) -> Result<String, sqlx::Error> {
    let mut result = String::new();
    if mode {
        let rows = sqlx::query(AssertSqlSafe(user_query))
        .fetch_all(&connection)
        .await?;

        for row in rows {
            result.push_str(&format!("{:?}\n", row));
        }

    } else {
        sqlx::query(AssertSqlSafe(user_query))
            .execute(&connection)
            .await?;

        result.push_str("(Ok): Operation was successful.");
    }
    Ok(result)
}