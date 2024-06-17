use async_std::task::spawn_local;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;
use crate::utils::DataType;

#[derive(Deserialize, Serialize)]
pub struct SubmitDataArgs {
    datatype: DataType,
    timestamp: DateTime<Utc>,
    value: f64,
    unit: String,
}

impl SubmitDataArgs {
    pub fn new(data_type: DataType, timestamp: DateTime<Utc>, value: f64, unit: String) -> Self {
        Self {
            datatype: data_type,
            timestamp,
            value,
            unit,
        }
    }
}

pub fn submit_data(data_type: DataType, timestamp: DateTime<Utc>, value: f64, unit: String) {
    spawn_local(async move {
        let args = to_value(&SubmitDataArgs::new(data_type, timestamp, value, unit)).unwrap();
        invoke("submit_data", args).await;
    });
}
