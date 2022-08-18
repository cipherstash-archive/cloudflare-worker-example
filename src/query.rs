use chrono::{DateTime, Utc};
use cipherstash_client::{api::QueryInput, indexer::query::*, record::Value};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "op", rename_all = "kebab-case")]
enum NameCondition {
    Eq { value: String },
}

impl From<NameCondition> for Query {
    fn from(condition: NameCondition) -> Self {
        match condition {
            NameCondition::Eq { value } => Query::Basic {
                index_name: "name".to_string(),
                kind: QueryKind::Exact {
                    value: value.into(),
                },
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "op", rename_all = "kebab-case")]
enum EmailCondition {
    Eq { value: String },
    Match { value: String },
}

impl From<EmailCondition> for Query {
    fn from(condition: EmailCondition) -> Self {
        match condition {
            EmailCondition::Eq { value } => Query::Basic {
                index_name: "exactEmail".to_string(),
                kind: QueryKind::Exact {
                    value: value.into(),
                },
            },
            EmailCondition::Match { value } => Query::Basic {
                index_name: "email".to_string(),
                kind: QueryKind::Match {
                    value: value.into(),
                },
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "op", rename_all = "kebab-case")]
enum DobCondition {
    Eq {
        value: DateTime<Utc>,
    },
    Lt {
        value: DateTime<Utc>,
    },
    Lte {
        value: DateTime<Utc>,
    },
    Gt {
        value: DateTime<Utc>,
    },
    Gte {
        value: DateTime<Utc>,
    },
    Between {
        min: DateTime<Utc>,
        max: DateTime<Utc>,
    },
}

impl From<DobCondition> for Query {
    fn from(condition: DobCondition) -> Self {
        Query::Basic {
            index_name: "dob".to_string(),
            kind: QueryKind::Range {
                value: match condition {
                    DobCondition::Eq { value } => RangeValue::Single {
                        operator: RangeOperator::Eq,
                        value: Value::Date(value.timestamp_millis() as f64),
                    },

                    DobCondition::Lt { value } => RangeValue::Single {
                        operator: RangeOperator::Lt,
                        value: Value::Date(value.timestamp_millis() as f64),
                    },

                    DobCondition::Lte { value } => RangeValue::Single {
                        operator: RangeOperator::Lte,
                        value: Value::Date(value.timestamp_millis() as f64),
                    },

                    DobCondition::Gt { value } => RangeValue::Single {
                        operator: RangeOperator::Gt,
                        value: Value::Date(value.timestamp_millis() as f64),
                    },

                    DobCondition::Gte { value } => RangeValue::Single {
                        operator: RangeOperator::Gte,
                        value: Value::Date(value.timestamp_millis() as f64),
                    },

                    DobCondition::Between { min, max } => RangeValue::Between {
                        min: Value::Date(min.timestamp_millis() as f64),
                        max: Value::Date(max.timestamp_millis() as f64),
                    },
                },
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserQuery {
    name: Option<NameCondition>,
    email: Option<EmailCondition>,
    dob: Option<DobCondition>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl From<UserQuery> for QueryInput {
    fn from(user_query: UserQuery) -> Self {
        let mut conditions: Vec<Query> = vec![];

        if let Some(name) = user_query.name {
            conditions.push(name.into());
        }

        if let Some(email) = user_query.email {
            conditions.push(email.into());
        }

        if let Some(dob) = user_query.dob {
            conditions.push(dob.into());
        }

        let query = if conditions.len() == 1 {
            conditions
                .into_iter()
                .next()
                .expect("Expected conditions array to have one value")
        } else {
            Query::Conjunctive(ConjunctiveCondition::All { conditions })
        };

        QueryInput {
            query,
            limit: user_query.limit,
            offset: user_query.offset,
        }
    }
}
