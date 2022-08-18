use chrono::{DateTime, TimeZone, Utc};
use cipherstash_client::collection;
use cipherstash_client::record::{Record, Value};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

// TODO: Let's make this User (more realistic)
// TODO: Also if Record had a Uuid this would be better
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    id: Uuid,
    name: String,
    email: String,
    dob: DateTime<Utc>,
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Self {
            id: *user.id.as_bytes(),
            fields: collection!(
                "name" => user.name,
                "email" => user.email,
                "dob" => Value::Date(user.dob.timestamp() as f64)
            ),
        }
    }
}

// TODO: I hate to say it but a serde style trait macro would be handy here
impl TryFrom<Record> for User {
    type Error = &'static str;

    fn try_from(record: Record) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_bytes(record.id),
            name: record
                .get("name")
                .and_then(|x| match x {
                    Value::String(x) => Some(x.to_string()),
                    _ => None,
                })
                .ok_or("Record was missing 'name' field")?,
            email: record
                .get("email")
                .and_then(|x| match x {
                    Value::String(x) => Some(x.to_string()),
                    _ => None,
                })
                .ok_or("Record was missing 'email' field")?,
            dob: record
                .get("dob")
                .and_then(|x| match x {
                    Value::Date(x) | Value::Float64(x) => Some(Utc.timestamp_millis(*x as _)),
                    _ => None,
                })
                .ok_or("Record was missing 'dob' field")?,
        })
    }
}
