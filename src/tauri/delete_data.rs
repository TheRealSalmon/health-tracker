use async_std::task::spawn_local;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;

#[derive(Deserialize, Serialize)]
pub struct DeleteDataArgs {
    timestamp: DateTime<Utc>,
}

impl DeleteDataArgs {
    pub fn new(timestamp: DateTime<Utc>) -> Self {
        Self { timestamp }
    }
}

pub fn delete_data(timestamp: DateTime<Utc>) {
    spawn_local(async move {
        let args = to_value(&DeleteDataArgs::new(timestamp)).unwrap();
        invoke("delete_data", args).await;
    });
}
