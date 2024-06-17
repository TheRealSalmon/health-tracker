use async_std::task::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::models::Data;
use crate::tauri::invoke;
use crate::utils::DataType;

#[derive(Deserialize, Serialize)]
pub struct GetDataArgs {
    data_type: String,
}

impl GetDataArgs {
    pub fn new(data_type: DataType) -> Self {
        Self {
            data_type: data_type.to_string(),
        }
    }
}

pub async fn get_data(data_type: DataType) -> Vec<Data> {
    let s = spawn_local(async move {
        let args = to_value(&GetDataArgs::new(data_type)).unwrap();
        invoke("get_data", args).await.as_string().unwrap()
    })
    .await;

    serde_json::from_str(&s).unwrap()
}
