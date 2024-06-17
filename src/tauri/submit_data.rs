use async_std::task::spawn_local;
use chrono::{DateTime, Utc};
use serde_wasm_bindgen::to_value;

use crate::models::Data;
use crate::tauri::invoke;
use crate::utils::DataType;

pub fn submit_data(data_type: DataType, timestamp: DateTime<Utc>, value: f64, unit: String) {
    spawn_local(async move {
        let args = to_value(&Data::new(data_type, timestamp, value, unit)).unwrap();
        invoke("submit_data", args).await;
    });
}
