use sqlx::{postgres::PgRow, types::time::OffsetDateTime, Row};

pub mod individuals;
pub mod relationships;
pub mod users;

pub fn get_possible_null_date(row: &PgRow, field: &str) -> String {
    let now = OffsetDateTime::now_utc();

    let db_date: OffsetDateTime = row.try_get(field).unwrap_or(now);

    if db_date == now {
        return String::from("null");
    } else {
        return db_date.to_string();
    }
}
