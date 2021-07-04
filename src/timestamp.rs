use std::convert::{From, Into};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Timestamp(prost_types::Timestamp);

impl From<DateTime<Utc>> for Timestamp {
    fn from(v: DateTime<Utc>) -> Self {
        Self(prost_types::Timestamp {
            seconds: v.timestamp(),
            nanos: v.timestamp_subsec_nanos() as i32,
        })
    }
}

impl Into<prost_types::Timestamp> for Timestamp {
    fn into(self: Self) -> prost_types::Timestamp {
        self.0
    }
}
