pub mod models;
pub mod schema;

use chrono::{NaiveDate, ParseError};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

pub fn parse_date_str_option(date_str: Option<&str>) -> Result<Option<NaiveDate>, ParseError> {
    date_str.map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
    .transpose()
}