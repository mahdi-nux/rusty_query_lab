use sqlx::{
    AssertSqlSafe, 
    Column, 
    Row, 
    SqlitePool, 
    TypeInfo, 
    ValueRef, 
    sqlite::{SqliteColumn, SqliteRow, SqliteValueRef}
};
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement};

fn datatype_detection(
    row: &SqliteRow, 
    column: &SqliteColumn, 
    value: SqliteValueRef<'_>) -> String {
    match value.type_info().name() {
        "INTEGER" => {
            row.get::<i64, _>(column.name()).to_string()
        }
        "REAL" => {
            row.get::<f64, _>(column.name()).to_string()
        }
        "TEXT" => {
            row.get::<String, _>(column.name())
        }
        "BLOB" => {
            let blob = row.get::<Vec<u8>, _>(column.name());
            let mut result = String::new();
            for byte in blob {
                result.push_str(&format!("{:02X} ", byte));
            }
            result
        }
        _ => "Error".to_string(),
    }
}

pub async fn init(address: String) -> Result<SqlitePool, sqlx::Error> {
    let my_pool = SqlitePool::connect(&format!("sqlite://{}", address)).await?;

    println!("DATABASE URL: {}", format!("sqlite://{}", address));
    Ok(my_pool)
}

pub async fn run_query(
    connection: SqlitePool,
    user_query: String,
    mode: bool
) -> Result<(Table, String), sqlx::Error> {
    let mut table = Table::new();
    table
        .load_style(UTF8_FULL.with_rounded_corners())
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(150);
    
    if mode {
        let rows = sqlx::query(AssertSqlSafe(user_query))
        .fetch_all(&connection)
        .await?;

        if let Some(first_row) = rows.first() {
            let mut table_header: Vec<&str> = Vec::new();

            for column in first_row.columns().iter() {
                table_header.push(column.name());
            }
            table.set_header(&table_header);
        }
        for row in rows {
            let mut table_rows: Vec<String> = Vec::new();

            for column in row.columns().iter() {
                let raw_data = row.try_get_raw(column.name())?;
                table_rows.push(datatype_detection(&row, column, raw_data));
            }
            table.add_row(&table_rows);
        }
    } else {
        sqlx::query(AssertSqlSafe(user_query))
            .execute(&connection)
            .await?;
    }
    Ok(
        (
            table,
            "(Ok): Operation was successful.".to_string()
        )
    )
}