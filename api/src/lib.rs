use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::types::time::PrimitiveDateTime;
use tower_cookies::cookie::time::format_description;

pub mod error;
pub mod http;

pub fn pdt_to_dt(pdt: &PrimitiveDateTime) -> DateTime<Utc> {
    let formatter =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

    let formatted_date_str = pdt.format(&formatter).unwrap();

    NaiveDateTime::parse_from_str(&formatted_date_str, "%F %T")
        .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
        .unwrap()
}
