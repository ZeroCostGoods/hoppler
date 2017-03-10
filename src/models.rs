use schema::events;
use diesel::expression::Expression;
use diesel::types::{Nullable, BigInt, Integer, Timestamp};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Insertable)]
#[table_name = "events"]
pub struct JsonEvent {
    pub timestamp: i64,
    pub session_id: String,
    pub time_on_page: Option<i32>,
    pub username: String,
    pub event_type: String,
    pub hostname: String,
    pub pathname: String,
    pub search: Option<String>,
    pub in_focus: bool,
    pub prior_hostname: Option<String>,
    pub prior_pathname: Option<String>,
    pub prior_search: Option<String>
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct DbEvent {
    pub id: i32,
    pub timestamp: i64,
    pub session_id: String,
    pub time_on_page: Option<i32>,
    pub username: String,
    pub event_type: String,
    pub hostname: String,
    pub pathname: String,
    pub search: Option<String>,
    pub in_focus: bool,
    pub prior_hostname: Option<String>,
    pub prior_pathname: Option<String>,
    pub prior_search: Option<String>
}

#[derive(Deserialize)]
pub struct EventsList {
    pub events: Vec<JsonEvent>
}