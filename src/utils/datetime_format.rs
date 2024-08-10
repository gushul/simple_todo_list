use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use serde::{self, Deserialize, Serializer, Deserializer};

const FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn serialize<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
        .and_then(|ndt| Utc.from_local_datetime(&ndt)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Ambiguous local time")))
}
