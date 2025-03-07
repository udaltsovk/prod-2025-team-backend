use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn datetime_into_timestamp(datetime: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: datetime.timestamp(),
        nanos: 0,
    }
}

pub fn timestamp_into_datetime(timestamp: Timestamp) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp.seconds, timestamp.nanos as u32)
}
