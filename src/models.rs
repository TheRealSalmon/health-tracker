use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::DataType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data {
    pub data_type: DataType,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub unit: String,
}

impl Data {
    pub fn new(data_type: DataType, timestamp: DateTime<Utc>, value: f64, unit: String) -> Self {
        Self {
            data_type,
            timestamp,
            value,
            unit,
        }
    }
}
