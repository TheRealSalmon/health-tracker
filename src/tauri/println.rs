use leptos::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;

#[derive(Deserialize, Serialize)]
pub struct PrintlnArgs {
    s: String,
}

#[allow(dead_code)]
pub fn println(s: String) {
    spawn_local(async move {
        let args = to_value(&PrintlnArgs { s }).unwrap();
        invoke("println", args).await;
    });
}
