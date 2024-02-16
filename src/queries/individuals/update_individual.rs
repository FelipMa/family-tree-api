use crate::database::get_connection_pool;
use crate::database::models::Individual;
use crate::queries::get_possible_null_date;
use sqlx::{
    postgres::PgArguments,
    query::Query,
    types::{time::OffsetDateTime, Uuid},
    Postgres, Row,
};
use std::collections::HashMap;
use time::format_description::well_known::Iso8601;

enum IndividualField {
    Name(String),
    Image(String),
    Birthday(OffsetDateTime),
    Generation(i32),
    IsAlive(bool),
    Death(OffsetDateTime),
}

pub async fn update_individual(
    id: String,
    name: Option<String>,
    image: Option<String>,
    birthday: Option<String>,
    generation: Option<i32>,
    is_alive: Option<bool>,
    death: Option<String>,
) -> Result<Individual, sqlx::Error> {
    let mut fields = HashMap::new();

    if let Some(name) = name {
        fields.insert("name", IndividualField::Name(name));
    }

    if let Some(image) = image {
        fields.insert("image", IndividualField::Image(image));
    }

    if let Some(birthday) = birthday {
        let birthday = match OffsetDateTime::parse(&birthday, &Iso8601::DEFAULT) {
            Ok(birthday) => birthday,
            Err(_) => return Err(sqlx::Error::Protocol("Invalid birthday".to_string())),
        };

        fields.insert("birthday", IndividualField::Birthday(birthday));
    }

    if let Some(generation) = generation {
        fields.insert("generation", IndividualField::Generation(generation));
    }

    if let Some(is_alive) = is_alive {
        fields.insert("is_alive", IndividualField::IsAlive(is_alive));
    }

    if let Some(death) = death {
        let death = match OffsetDateTime::parse(&death, &Iso8601::DEFAULT) {
            Ok(death) => death,
            Err(_) => return Err(sqlx::Error::Protocol("Invalid death".to_string())),
        };

        fields.insert("death", IndividualField::Death(death));
    }

    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid ID".to_string())),
    };

    let mut count = 0;

    let query = format!(
        "UPDATE \"individuals\" SET {} WHERE id = {} RETURNING *",
        fields
            .iter()
            .map(|(key, _value)| {
                count += 1;
                return format!("\"{}\" = ${}", key, count);
            })
            .collect::<Vec<String>>()
            .join(", "),
        format!("${}", count + 1)
    );

    println!("{}", query);

    let pool = get_connection_pool().await?;

    let mut sqlx_query: Query<'_, Postgres, PgArguments> = sqlx::query(&query);

    for field in fields.values() {
        match field {
            IndividualField::Name(name) => {
                sqlx_query = sqlx_query.bind(name);
            }
            IndividualField::Image(image) => {
                sqlx_query = sqlx_query.bind(image);
            }
            IndividualField::Birthday(birthday) => {
                sqlx_query = sqlx_query.bind(birthday);
            }
            IndividualField::Generation(generation) => {
                sqlx_query = sqlx_query.bind(generation);
            }
            IndividualField::IsAlive(is_alive) => {
                sqlx_query = sqlx_query.bind(is_alive);
            }
            IndividualField::Death(death) => {
                sqlx_query = sqlx_query.bind(death);
            }
        }
    }

    sqlx_query = sqlx_query.bind(id);

    let row = sqlx_query.fetch_one(&pool).await?;

    let individual = Individual {
        id: row.get::<Uuid, _>("id").to_string(),
        name: row.get("name"),
        image: row.try_get("image").unwrap_or("null".to_string()),
        birthday: get_possible_null_date(&row, "birthday"),
        generation: row.get("generation"),
        is_alive: row.get("is_alive"),
        death: get_possible_null_date(&row, "death"),
        created_at: row.get::<OffsetDateTime, _>("created_at").to_string(),
        updated_at: row.get::<OffsetDateTime, _>("updated_at").to_string(),
    };

    return Ok(individual);
}
