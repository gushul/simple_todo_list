use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::utils::datetime_format;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,

    #[serde(with = "datetime_format")]
    pub date: DateTime<Utc>,

    pub category: String,

    pub status: bool,
}
