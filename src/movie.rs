use cipherstash_client::collection;
use cipherstash_client::record::{Record, Value};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

// TODO: Let's make this User (more realistic)
// TODO: Also if Record had a Uuid this would be better
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Movie {
    id: Uuid,
    title: String,
    #[serde(rename = "runningTime")]
    running_time: u64,
}

impl From<Movie> for Record {
    fn from(movie: Movie) -> Self {
        Self {
            id: *movie.id.as_bytes(),
            fields: collection!(
                "title" => movie.title,
                "runningTime" => movie.running_time
            ),
        }
    }
}

// TODO: I hate to say it but a serde style trait macro would be handy here
impl TryFrom<Record> for Movie {
    type Error = &'static str;

    fn try_from(record: Record) -> std::result::Result<Self, Self::Error> {
        if let Some(Value::String(title)) = record.get("title") {
            if let Some(Value::Uint64(running_time)) = record.get("runningTime") {
                return Ok(Movie {
                    id: Uuid::from_bytes(record.id),
                    title: title.to_string(),
                    running_time: *running_time,
                });
            }
        }

        Err("Missing fields on Movie Record")
    }
}
